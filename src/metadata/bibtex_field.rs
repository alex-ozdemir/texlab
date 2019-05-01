pub struct BibtexField {
    pub name: &'static str,
    pub documentation: &'static str,
}

pub fn get_documentation(name: &str) -> Option<&'static str> {
    BIBTEX_FIELDS
        .iter()
        .find(|field| field.name == name)
        .map(|field| field.documentation)
}

pub static BIBTEX_FIELDS: &'static [BibtexField] = &[
    BibtexField {
        name: "abstract",
        documentation: "This field is intended for recording abstracts in a bib file, to be printed by a special bibliography style. It is not used by all standard bibliography styles."
    },
    BibtexField {
        name: "addendum",
        documentation: "Miscellaneous bibliographic data to be printed at the end of the entry. This is similar to the `note` field except that it is printed at the end of the bibliography entry."
    },
    BibtexField {
        name: "afterword",
        documentation: "The author(s) of an afterword to the work. If the author of the afterword is identical to the `editor` and/or `translator`, the standard styles will automatically concatenate these fields in the bibliography. See also `introduction` and `foreword`."
    },
    BibtexField {
        name: "annotation",
        documentation: "This field may be useful when implementing a style for annotated bibliographies. It is not used by all standard bibliography styles. Note that this field is completely unrelated to `annotator`. The `annotator` is the author of annotations which are part of the work cited."
    },
    BibtexField {
        name: "annotator",
        documentation: "The author(s) of annotations to the work. If the annotator is identical to the `editor` and/or `translator`, the standard styles will automatically concatenate these fields in the bibliography. See also `commentator`."
    },
    BibtexField {
        name: "author", 
        documentation: "The author(s) of the `title`." 
    },
    BibtexField {
        name: "authortype",
        documentation: "The type of author. This field will affect the string (if any) used to introduce the author. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "bookauthor",
        documentation: "The author(s) of the `booktitle`."
    },
    BibtexField {
        name: "bookpagination",
        documentation: "If the work is published as part of another one, this is the pagination scheme of the enclosing work, i. e., `bookpagination` relates to `pagination` like `booktitle` to `title`. The value of this field will affect the formatting of the `pages` and `pagetotal` fields. The key should be given in the singular form. Possible keys are `page`, `column`, `line`, `verse`, `section`, and `paragraph`. See also `pagination`."
    },
    BibtexField {
        name: "booksubtitle",
        documentation: "The subtitle related to the `booktitle`. If the subtitle field refers to a work which is part of a larger publication, a possible subtitle of the main work is given in this field. See also `subtitle`."
    },
    BibtexField {
        name: "booktitle",
        documentation: "If the `title` field indicates the title of a work which is part of a larger publication, the title of the main work is given in this field. See also `title`."
    },
    BibtexField {
        name: "booktitleaddon",
        documentation: "An annex to the `booktitle`, to be printed in a different font."
    },
    BibtexField {
        name: "chapter",
        documentation: "A chapter or section or any other unit of a work."
    },
    BibtexField {
        name: "commentator",
        documentation: "The author(s) of a commentary to the work. Note that this field is intended for commented editions which have a commentator in addition to the author. If the work is a stand-alone commentary, the commentator should be given in the `author` field. If the commentator is identical to the `editor` and/or `translator`, the standard styles will automatically concatenate these fields in the bibliography. See also `annotator`."
    },
    BibtexField {
        name: "date",
        documentation: "The publication date. See also `month` and `year`."
    },
    BibtexField {
        name: "doi",
        documentation: "The Digital Object Identifier of the work."
    },
    BibtexField {
        name: "edition",
        documentation: "The edition of a printed publication. This must be an integer, not an ordinal. Don’t say `edition={First}` or `edition={1st}` but `edition={1}`. The bibliography style converts this to a language dependent ordinal. It is also possible to give the edition as a literal string, for example \"Third, revised and expanded edition\"."
    },
    BibtexField {
        name: "editor",
        documentation: "The editor(s) of the `title`, `booktitle`, or `maintitle`, depending on the entry type. Use the `editortype` field to specify the role if it is different from `editor`."
    },
    BibtexField {
        name: "editora",
        documentation: "A secondary editor performing a different editorial role, such as compiling, redacting, etc. Use the `editoratype` field to specify the role."
    },
    BibtexField {
        name: "editorb",
        documentation: "Another secondary editor performing a different role. Use the `editorbtype` field to specify the role."
    },
    BibtexField {
        name: "editorc",
        documentation: "Another secondary editor performing a different role. Use the `editorctype` field to specify the role."
    },
    BibtexField {
        name: "editortype",
        documentation: "The type of editorial role performed by the `editor`. Roles supported by default are `editor`, `compiler`, `founder`, `continuator`, `redactor`, `reviser`, `collaborator`, `organizer`. The role `editor` is the default. In this case, the field is omissible."
    },
    BibtexField {
        name: "editoratype",
        documentation: "Similar to `editortype` but referring to the `editora` field."
    },
    BibtexField {
        name: "editorbtype",
        documentation: "Similar to `editortype` but referring to the `editorb` field."
    },
    BibtexField {
        name: "editorctype",
        documentation: "Similar to `editortype` but referring to the `editorc` field."
    },
    BibtexField {
        name: "eid",
        documentation: "The electronic identifier of an `@article`."
    },
    BibtexField {
        name: "entrysubtype",
        documentation: "This field, which is not used by the standard styles, may be used to specify a subtype of an entry type. This may be useful for bibliography styles which support a finergrained set of entry types."
    },
    BibtexField {
        name: "eprint",
        documentation: "The electronic identifier of an online publication. This is roughly comparable to a doi but specific to a certain archive, repository, service, or system. See also `eprinttype` and `eprintclass`."
    },
    BibtexField {
        name: "eprintclass",
        documentation: "Additional information related to the resource indicated by the `eprinttype` field. This could be a section of an archive, a path indicating a service, a classification of some sort, etc. See also`eprint` and `eprinttype`."
    },
    BibtexField {
        name: "eprinttype",
        documentation: "The type of `eprint` identifier, e. g., the name of the archive, repository, service, or system the `eprint` field refers to. See also `eprint` and `eprintclass`."
    },
    BibtexField {
        name: "eventdate",
        documentation: "The date of a conference, a symposium, or some other event in `@proceedings` and `@inproceedings` entries. See also `eventtitle` and `venue`."
    },
    BibtexField {
        name: "eventtitle",
        documentation: "The title of a conference, a symposium, or some other event in `@proceedings` and `@inproceedings` entries. Note that this field holds the plain title of the event. Things like \"Proceedings of the Fifth XYZ Conference\" go into the `titleaddon` or `booktitleaddon` field, respectively. See also `eventdate` and `venue`."
    },
    BibtexField {
        name: "eventtitleaddon",
        documentation: "An annex to the `eventtitle` field. Can be used for known event acronyms, for example."
    },
    BibtexField {
        name: "file",
        documentation: "A local link to a PDF or other version of the work. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "foreword",
        documentation: "The author(s) of a foreword to the work. If the author of the foreword is identical to the `editor` and/or `translator`, the standard styles will automatically concatenate these fields in the bibliography. See also `introduction` and `afterword`."
    },
    BibtexField {
        name: "holder",
        documentation: "The holder(s) of a `@patent`, if different from the `author`. Note that corporate holders need to be wrapped in an additional set of braces."
    },
    BibtexField {
        name: "howpublished",
        documentation: "A publication notice for unusual publications which do not fit into any of the common categories."
    },
    BibtexField {
        name: "indextitle",
        documentation: "A title to use for indexing instead of the regular `title` field. This field may be useful if you have an entry with a title like \"An Introduction to …\" and want that indexed as \"Introduction to …, An\". Style authors should note that `biblatex` automatically copies the value of the `title` field to `indextitle` if the latter field is undefined."
    },
    BibtexField {
        name: "institution",
        documentation: "The name of a university or some other institution, depending on the entry type. Traditional BibTeX uses the field name `school` for theses, which is supported as an alias."
    },
    BibtexField {
        name: "introduction",
        documentation: "The author(s) of an introduction to the work. If the author of the introduction is identical to the `editor` and/or `translator`, the standard styles will automatically concatenate these fields in the bibliography. See also `foreword` and `afterword`."
    },
    BibtexField {
        name: "isan",
        documentation: "The International Standard Audiovisual Number of an audiovisual work. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "isbn",
        documentation: "The International Standard Book Number of a book."
    },
    BibtexField {
        name: "ismn",
        documentation: "The International Standard Music Number for printed music such as musical scores. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "isrn",
        documentation: "The International Standard Technical Report Number of a technical report."
    },
    BibtexField {
        name: "issn",
        documentation: "The International Standard Serial Number of a periodical."
    },
    BibtexField {
        name: "issue",
        documentation: "The issue of a journal. This field is intended for journals whose individual issues are identified by a designation such as ‘Spring’ or ‘Summer’ rather than the month or a number. The placement of `issue` is similar to `month` and `number`, integer ranges and short designators are better written to the number field. See also `month` and `number`."
    },
    BibtexField {
        name: "issuesubtitle",
        documentation: "The subtitle of a specific issue of a journal or other periodical."
    },
    BibtexField {
        name: "issuetitle",
        documentation: "The title of a specific issue of a journal or other periodical."
    },
    BibtexField {
        name: "iswc",
        documentation: "The International Standard Work Code of a musical work. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "journalsubtitle",
        documentation: "The subtitle of a journal, a newspaper, or some other periodical."
    },
    BibtexField {
        name: "journaltitle",
        documentation: "The name of a journal, a newspaper, or some other periodical."
    },
    BibtexField {
        name: "label",
        documentation: "A designation to be used by the citation style as a substitute for the regular label if any data required to generate the regular label is missing. For example, when an author-year citation style is generating a citation for an entry which is missing the author or the year, it may fall back to `label`. Note that, in contrast to `shorthand`, `label` is only used as a fallback. See also `shorthand`."
    },
    BibtexField {
        name: "language",
        documentation: "The language(s) of the work. Languages may be specified literally or as localisation keys. If localisation keys are used, the prefix lang is omissible. See also `origlanguage`."
    },
    BibtexField {
        name: "library",
        documentation: "This field may be useful to record information such as a library name and a call number. This may be printed by a special bibliography style if desired. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "location",
        documentation: "The place(s) of publication, i. e., the location of the `publisher` or `institution`, depending on the entry type. Traditional BibTeX uses the field name `address`, which is supported as an alias. With `@patent` entries, this list indicates the scope of a patent."
    },
    BibtexField {
        name: "mainsubtitle",
        documentation: "The subtitle related to the `maintitle`. See also `subtitle`."
    },
    BibtexField {
        name: "maintitle",
        documentation: "The main title of a multi-volume book, such as *Collected Works*. If the `title` or `booktitle` field indicates the title of a single volume which is part of multi-volume book, the title of the complete work is given in this field."
    },
    BibtexField {
        name: "maintitleaddon",
        documentation: "An annex to the `maintitle`, to be printed in a different font."
    },
    BibtexField {
        name: "month",
        documentation: "The publication month. This must be an integer, not an ordinal or a string. Don’t say `month={January}` but `month={1}`. The bibliography style converts this to a language dependent string or ordinal where required. This field is a literal field only when given explicitly in the data (for plain BibTeX compatibility for example). It is however better to use the `date` field as this supports many more features."
    },
    BibtexField {
        name: "nameaddon",
        documentation: "An addon to be printed immediately after the author name in the bibliography. Not used by the standard bibliography styles. This field may be useful to add an alias or pen name (or give the real name if the pseudonym is commonly used to refer to that author)."
    },
    BibtexField {
        name: "note",
        documentation: "Miscellaneous bibliographic data which does not fit into any other field. The note field may be used to record bibliographic data in a free format. Publication facts such as \"Reprint of the edition London 1831\" are typical candidates for the note field. See also `addendum`."
    },
    BibtexField {
        name: "number",
        documentation: "The number of a journal or the volume/number of a book in a `series`. See also `issue`. With `@patent` entries, this is the number or record token of a patent or patent request. Normally this field will be an integer or an integer range, but in certain cases it may also contain \"S1\", \"Suppl. 1\", in these cases the output should be scrutinised carefully."
    },
    BibtexField {
        name: "organization",
        documentation: "The organization(s) that published a `@manual` or an `@online` resource, or sponsored a conference."
    },
    BibtexField {
        name: "origdate",
        documentation: "If the work is a translation, a reprint, or something similar, the publication date of the original edition. Not used by the standard bibliography styles. See also `date`."
    },
    BibtexField {
        name: "origlanguage",
        documentation: "If the work is a translation, the language(s) of the original work. See also `language`."
    },
    BibtexField {
        name: "origlocation",
        documentation: "If the work is a translation, a reprint, or something similar, the location of the original edition. Not used by the standard bibliography styles. See also `location`."
    },
    BibtexField {
        name: "origpublisher",
        documentation: "If the work is a translation, a reprint, or something similar, the publisher of the original edition. Not used by the standard bibliography styles. See also `publisher`."
    },
    BibtexField {
        name: "origtitle",
        documentation: "If the work is a translation, the `title` of the original work. Not used by the standard bibliography styles. See also `title`."
    },
    BibtexField {
        name: "pages",
        documentation: "One or more page numbers or page ranges. If the work is published as part of another one, such as an article in a journal or a collection, this field holds the relevant page range in that other work. It may also be used to limit the reference to a specific part of a work (a chapter in a book, for example)."
    },
    BibtexField {
        name: "pagetotal",
        documentation: "The total number of pages of the work."
    },
    BibtexField {
        name: "pagination",
        documentation: "The pagination of the work. The value of this field will affect the formatting the *postnote* argument to a citation command. The key should be given in the singular form. Possible keys are `page`, `column`, `line`, `verse`, `section`, and `paragraph`. See also `bookpagination`."
    },
    BibtexField {
        name: "part",
        documentation: "The number of a partial volume. This field applies to books only, not to journals. It may be used when a logical volume consists of two or more physical ones. In this case the number of the logical volume goes in the `volume` field and the number of the part of that volume in the `part` field. See also `volume`."
    },
    BibtexField {
        name: "publisher",
        documentation: "The name(s) of the publisher(s)."
    },
    BibtexField {
        name: "pubstate",
        documentation: "The publication state of the work, e. g., 'in press'."
    },
    BibtexField {
        name: "reprinttitle",
        documentation: "The title of a reprint of the work. Not used by the standard styles."
    },
    BibtexField {
        name: "series",
        documentation: "The name of a publication series, such as \"Studies in …\", or the number of a journal series. Books in a publication series are usually numbered. The number or volume of a book in a series is given in the `number` field. Note that the `@article` entry type makes use of the `series` field as well, but handles it in a special way."
    },
    BibtexField {
        name: "shortauthor",
        documentation: "The author(s) of the work, given in an abbreviated form. This field is mainly intended for abbreviated forms of corporate authors."
    },
    BibtexField {
        name: "shorteditor",
        documentation: "The editor(s) of the work, given in an abbreviated form. This field is mainly intended for abbreviated forms of corporate editors."
    },
    BibtexField {
        name: "shorthand",
        documentation: "A special designation to be used by the citation style instead of the usual label. If defined, it overrides the default label. See also `label`."
    },
    BibtexField {
        name: "shorthandintro",
        documentation: "The verbose citation styles which comes with this package use a phrase like \"henceforth cited as [shorthand]\" to introduce shorthands on the first citation. If the `shorthandintro` field is defined, it overrides the standard phrase. Note that the alternative phrase must include the shorthand."
    },
    BibtexField {
        name: "shortjournal",
        documentation: "A short version or an acronym of the `journaltitle`. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "shortseries",
        documentation: "A short version or an acronym of the `series` field. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "shorttitle",
        documentation: "The title in an abridged form. This field is usually not included in the bibliography. It is intended for citations in author-title format. If present, the author-title citation styles use this field instead of `title`."
    },
    BibtexField {
        name: "subtitle",
        documentation: "The subtitle of the work."
    },
    BibtexField {
        name: "title",
        documentation: "The title of the work."
    },
    BibtexField {
        name: "titleaddon",
        documentation: "An annex to the `title`, to be printed in a different font."
    },
    BibtexField {
        name: "translator",
        documentation: "The translator(s) of the `title` or `booktitle`, depending on the entry type. If the translator is identical to the `editor`, the standard styles will automatically concatenate these fields in the bibliography."
    },
    BibtexField {
        name: "type",
        documentation: "The type of a `manual`, `patent`, `report`, or `thesis`."
    },
    BibtexField {
        name: "url",
        documentation: "The URL of an online publication. If it is not URL-escaped (no ‘%’ chars) it will be URI-escaped according to RFC 3987, that is, even Unicode chars will be correctly escaped."
    },
    BibtexField {
        name: "urldate",
        documentation: "The access date of the address specified in the `url` field."
    },
    BibtexField {
        name: "venue",
        documentation: "The location of a conference, a symposium, or some other event in `@proceedings` and `@inproceedings` entries. Note that the `location` list holds the place of publication. It therefore corresponds to the `publisher` and `institution` lists. The location of the event is given in the `venue` field. See also `eventdate` and `eventtitle`."
    },
    BibtexField {
        name: "version",
        documentation: "The revision number of a piece of software, a manual, etc."
    },
    BibtexField {
        name: "volume",
        documentation: "The volume of a multi-volume book or a periodical. It is expected to be an integer, not necessarily in arabic numerals since `biber` will automatically from roman numerals or arabic letter to integers internally for sorting purposes. See also `part`. See the `noroman` option which can be used to suppress roman numeral parsing. This can help in cases where there is an ambiguity between parsing as roman numerals or alphanumeric (e.g. ‘C’)."
    },
    BibtexField {
        name: "volumes",
        documentation: "The total number of volumes of a multi-volume work. Depending on the entry type, this field refers to `title` or `maintitle`. It is expected to be an integer, not necessarily in arabic numerals since `biber` will automatically from roman numerals or arabic letter to integers internally for sorting purposes. See the `noroman` option which can be used to suppress roman numeral parsing. This can help in cases where there is an ambiguity between parsing as roman numerals or alphanumeric (e.g. ‘C’)."
    },
    BibtexField {
        name: "year",
        documentation: "The year of publication. This field is a literal field only when given explicitly in the data (for plain BibTeX compatibility for example). It is however better to use the `date` field as this is compatible with plain years too and supports many more features."
    },
    BibtexField {
        name: "crossref",
        documentation: "This field holds an entry key for the cross-referencing feature. Child entries with a `crossref` field inherit data from the parent entry specified in the `crossref` field. If the number of child entries referencing a specific parent entry hits a certain threshold, the parent entry is automatically added to the bibliography even if it has not been cited explicitly. The threshold is settable with the `mincrossrefs` package option. Style authors should note that whether or not the `crossref` fields of the child entries are defined on the `biblatex` level depends on the availability of the parent entry. If the parent entry is available, the `crossref` fields of the child entries will be defined. If not, the child entries still inherit the data from the parent entry but their `crossref` fields will be undefined. Whether the parent entry is added to the bibliography implicitly because of the threshold or explicitly because it has been cited does not matter. See also the `xref` field."
    },
    BibtexField {
        name: "entryset",
        documentation: "This field is specific to entry sets. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "execute",
        documentation: "A special field which holds arbitrary TeX code to be executed whenever the data of the respective entry is accessed. This may be useful to handle special cases. Conceptually, this field is comparable to the hooks `AtEveryBibitem`, `AtEveryLositem`, and `AtEveryCitekey`, except that it is definable on a per-entry basis in the `bib` file. Any code in this field is executed automatically immediately after these hooks."
    },
    BibtexField {
        name: "gender",
        documentation: "The gender of the author or the gender of the editor, if there is no author. The following identifiers are supported: `sf` (feminine singular, a single female name), `sm` (masculine singular, a single male name), `sn` (neuter singular, a single neuter name), `pf` (feminine plural, a list of female names), `pm` (masculine plural, a list of male names), `pn` (neuter plural, a list of neuter names),`pp` (plural, a mixed gender list of names). This information is only required by special bibliography and citation styles and only in certain languages. For example, a citation style may replace recurrent author names with a term such as 'idem'. If the Latin word is used, as is custom in English and French, there is no need to specify the gender. In German publications, however, such key terms are usually given in German and in this case they are gender-sensitive."
    },
    BibtexField {
        name: "langid",
        documentation: "The language id of the bibliography entry. The alias `hyphenation` is provided for backwards compatibility. The identifier must be a language name known to the `babel/polyglossia` packages. This information may be used to switch hyphenation patterns and localise strings in the bibliography. Note that the language names are case sensitive. The languages currently supported by this package are given in table 2. Note that `babel` treats the identifier `english` as an alias for `british` or `american`, depending on the `babel` version. The `biblatex` package always treats it as an alias for `american`. It is preferable to use the language identifiers `american` and `british` (`babel`) or a language specific option to specify a language variant (`polyglossia`, using the `langidopts` field) to avoid any possible confusion."
    },
    BibtexField {
        name: "langidopts",
        documentation: "For `polyglossia` users, allows per-entry language specific options. The literal value of this field is passed to `polyglossia`’s language switching facility when using the package option `autolang=langname`."
    },
    BibtexField {
        name: "ids",
        documentation: "Citation key aliases for the main citation key. An entry may be cited by any of its aliases and `biblatex` will treat the citation as if it had used the primary citation key. This is to aid users who change their citation keys but have legacy documents which use older keys for the same entry. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "indexsorttitle",
        documentation: "The title used when sorting the index. In contrast to indextitle, this field is used for sorting only. The printed title in the index is the indextitle or the title field. This field may be useful if the title contains special characters or commands which interfere with the sorting of the index. Style authors should note that biblatex automatically copies the value of either the indextitle or the title field to indexsorttitle if the latter field is undefined."
    },
    BibtexField {
        name: "keywords",
        documentation: "A separated list of keywords. These keywords are intended for the bibliography filters, they are usually not printed. Note that with the default separator (comma), spaces around the separator are ignored."
    },
    BibtexField {
        name: "options",
        documentation: "A separated list of entry options in *key*=*value* notation. This field is used to set options on a per-entry basis. Note that citation and bibliography styles may define additional entry options."
    },
    BibtexField {
        name: "presort",
        documentation: "A special field used to modify the sorting order of the bibliography. This field is the first item the sorting routine considers when sorting the bibliography, hence it may be used to arrange the entries in groups. This may be useful when creating subdivided bibliographies with the bibliography filters. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "related",
        documentation: "Citation keys of other entries which have a relationship to this entry. The relationship is specified by the `relatedtype` field."
    },
    BibtexField {
        name: "relatedoptions",
        documentation: "Per-type options to set for a related entry. Note that this does not set the options on the related entry itself, only the `dataonly` clone which is used as a datasource for the parent entry."
    },
    BibtexField {
        name: "relatedtype",
        documentation: "An identifier which specified the type of relationship for the keys listed in the `related` field. The identifier is a localised bibliography string printed before the data from the related entry list. It is also used to identify type-specific formatting directives and bibliography macros for the related entries."
    },
    BibtexField {
        name: "relatedstring",
        documentation: "A field used to override the bibliography string specified by `relatedtype`."
    },
    BibtexField {
        name: "sortkey",
        documentation: "A field used to modify the sorting order of the bibliography. Think of this field as the master sort key. If present, `biblatex` uses this field during sorting and ignores everything else, except for the presort field. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "sortname",
        documentation: "A name or a list of names used to modify the sorting order of the bibliography. If present, this list is used instead of `author` or `editor` when sorting the bibliography. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "sortshorthand",
        documentation: "Similar to sortkey but used in the list of shorthands. If present, biblatex uses this field instead of shorthand when sorting the list of shorthands. This is useful if the shorthand field holds shorthands with formatting commands such as `emph` or `\textbf`. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "sorttitle",
        documentation: "A field used to modify the sorting order of the bibliography. If present, this field is used instead of the title field when sorting the bibliography. The sorttitle field may come in handy if you have an entry with a title like \"An Introduction to…\" and want that alphabetized under ‘I’ rather than ‘A’. In this case, you could put \"Introduction to…\" in the sorttitle field. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "sortyear",
        documentation: "A field used to modify the sorting order of the bibliography. In the default sorting templates, if this field is present, it is used instead of the year field when sorting the bibliography. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "xdata",
        documentation: "This field inherits data from one or more `@xdata` entries. Conceptually, the `xdata` field is related to crossref and xref: `crossref` establishes a logical parent/child relation and inherits data; `xref` establishes as logical parent/child relation without inheriting data; `xdata` inherits data without establishing a relation. The value of the `xdata` may be a single entry key or a separated list of keys. This field is consumed by the backend processing and does not appear in the `.bbl`."
    },
    BibtexField {
        name: "xref",
        documentation: "This field is an alternative cross-referencing mechanism. It differs from `crossref` in that the child entry will not inherit any data from the parent entry specified in the `xref` field. If the number of child entries referencing a specific parent entry hits a certain threshold, the parent entry is automatically added to the bibliography even if it has not been cited explicitly. The threshold is settable with the `minxrefs` package option. Style authors should note that whether or not the `xref` fields of the child entries are defined on the `biblatex` level depends on the availability of the parent entry. If the parent entry is available, the `xref` fields of the child entries will be defined. If not, their `xref` fields will be undefined. Whether the parent entry is added to the bibliography implicitly because of the threshold or explicitly because it has been cited does not matter. See also the `crossref` field."
    },
    BibtexField {
        name: "namea",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "nameb",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "namec",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "nameatype",
        documentation: "Similar to `authortype` and `editortype` but referring to the fields `name[a--c]`. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "namebtype",
        documentation: "Similar to `authortype` and `editortype` but referring to the fields `name[a--c]`. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "namectype",
        documentation: "Similar to `authortype` and `editortype` but referring to the fields `name[a--c]`. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "lista",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "listb",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "listc",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "listd",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "liste",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "listf",
        documentation: "Custom lists for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "usera",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "userb",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "userc",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "userd",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "usere",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "userf",
        documentation: "Custom fields for special bibliography styles. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "verba",
        documentation: "Similar to the custom fields except that these are verbatim fields. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "verbb",
        documentation: "Similar to the custom fields except that these are verbatim fields. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "verbc",
        documentation: "Similar to the custom fields except that these are verbatim fields. Not used by the standard bibliography styles."
    },
    BibtexField {
        name: "address",
        documentation: "An alias for `location`, provided for BibTeX compatibility. Traditional BibTeX uses the slightly misleading field name `address` for the place of publication, i. e., the location of the publisher, while `biblatex` uses the generic field name `location`."
    },
    BibtexField {
        name: "annote",
        documentation: "An alias for `annotation`, provided for jurabib compatibility."
    },
    BibtexField {
        name: "archiveprefix",
        documentation: "An alias for `eprinttype`, provided for arXiv compatibility."
    },
    BibtexField {
        name: "journal",
        documentation: "An alias for `journaltitle`, provided for BibTeX compatibility."
    },
    BibtexField {
        name: "key",
        documentation: "An alias for `sortkey`, provided for BibTeX compatibility."
    },
    BibtexField {
        name: "pdf",
        documentation: "An alias for `file`, provided for JabRef compatibility."
    },
    BibtexField {
        name: "primaryclass",
        documentation: "An alias for `eprintclass`, provided for arXiv compatibility."
    },
    BibtexField {
        name: "school",
        documentation: "An alias for `institution`, provided for BibTeX compatibility. The `institution` field is used by traditional BibTeX for technical reports whereas the `school` field holds the institution associated with theses. The `biblatex` package employs the generic field name `institution` in both cases."
    }
];
