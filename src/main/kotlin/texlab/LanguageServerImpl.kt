package texlab

import org.eclipse.lsp4j.*
import org.eclipse.lsp4j.jsonrpc.messages.Either
import org.eclipse.lsp4j.jsonrpc.services.JsonRequest
import org.eclipse.lsp4j.services.LanguageServer
import org.eclipse.lsp4j.services.TextDocumentService
import org.eclipse.lsp4j.services.WorkspaceService
import texlab.build.BuildParams
import texlab.build.BuildStatus
import java.io.IOException
import java.net.URI
import java.nio.file.FileSystems
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.util.concurrent.CompletableFuture

class LanguageServerImpl : LanguageServer {
    private val workspace: Workspace = Workspace()
    private val textDocumentService = TextDocumentServiceImpl(workspace)
    private val workspaceService = WorkspaceServiceImpl()
    private lateinit var client: LanguageClientExtensions

    fun connect(client: LanguageClientExtensions) {
        this.client = client
        this.textDocumentService.client = client
    }

    override fun initialize(params: InitializeParams): CompletableFuture<InitializeResult> {
        if (params.rootUri != null) {
            val root = URI.create(params.rootUri)
            synchronized(workspace) {
                loadWorkspace(root)
            }
        }

        val capabilities = ServerCapabilities().apply {
            val syncOptions = TextDocumentSyncOptions().apply {
                openClose = true
                change = TextDocumentSyncKind.Full
            }
            textDocumentSync = Either.forRight(syncOptions)
            documentSymbolProvider = true
            renameProvider = Either.forLeft(true)
            documentLinkProvider = DocumentLinkOptions(false)
            completionProvider = CompletionOptions(true, listOf("\\", "{", "}", "@"))
            foldingRangeProvider = Either.forLeft(true)
            definitionProvider = true
            hoverProvider = true
            documentFormattingProvider = true
            referencesProvider = true
        }

        return CompletableFuture.completedFuture(InitializeResult(capabilities))
    }

    override fun initialized(params: InitializedParams?) {
        client.setStatus(StatusParams(ServerStatus.IDLE, null))
    }

    private fun loadWorkspace(root: URI) {
        if (root.scheme == "file") {
            val matcher = FileSystems.getDefault().getPathMatcher("glob:**.{tex,sty,cls,bib}")
            Files.walk(Paths.get(root))
                    .filter { Files.isRegularFile(it) }
                    .filter { matcher.matches(it) }
                    .forEach { loadWorkspaceFile(it) }
        }
    }

    private fun loadWorkspaceFile(file: Path) {
        val extension = file.fileName.toFile().extension
        val language = getLanguageByExtension(extension) ?: return
        try {
            val text = Files.readAllBytes(file).toString(Charsets.UTF_8)
            val document = Document.create(file.toUri(), language)
            document.text = text
            document.analyze()
            workspace.documents.add(document)
        } catch (e: IOException) {
            e.printStackTrace()
        }
    }

    override fun getTextDocumentService(): TextDocumentService = textDocumentService

    override fun getWorkspaceService(): WorkspaceService = workspaceService

    override fun shutdown(): CompletableFuture<Any> {
        return CompletableFuture.completedFuture(null)
    }

    override fun exit() {
    }

    @JsonRequest("textDocument/build", useSegment = false)
    fun build(params: BuildParams): CompletableFuture<BuildStatus> {
        return textDocumentService.build(params)
    }
}
