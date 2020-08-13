use crate::disass::disassemble;
use dex;
use dex::code::CodeItem;
use dex::method::MethodIdItem;
use dex::Dex;
use std::convert::TryInto;

pub fn get_invoked_methods<'a>(
    code: &'a CodeItem,
    dex: &'a Dex<Vec<u8>>,
) -> impl Iterator<Item = MethodIdItem> + 'a {
    disassemble(code).filter_map(move |ins| {
        if ins.is_invoke() {
            let target = dex
                .get_method_item(ins.invoke_target().try_into().unwrap())
                .unwrap();
            Some(target)
        } else {
            None
        }
    })
}

pub fn get_invoked_methods_names<'a>(
    code: &'a CodeItem,
    dex: &'a Dex<Vec<u8>>,
) -> impl Iterator<Item = String> + 'a {
    get_invoked_methods(code, dex).map(move |target| {
        let method_name = dex.get_string(target.name_idx()).unwrap().to_string();
        let class_name = dex
            .get_type(target.class_idx().into())
            .unwrap()
            .type_descriptor()
            .to_string();
        format!("{}->{}", class_name, method_name)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apk::Apk;

    #[test]
    fn test_callgraph() {
        let calls = [
            "Lcom/github/mertakdut/Content;-><init>",
            "Lcom/github/mertakdut/Content;->setZipFilePath",
            "Ljava/util/zip/ZipFile;-><init>",
            "Ljava/util/zip/ZipFile;->entries",
            "Ljava/util/Enumeration;->hasMoreElements",
            "Ljava/util/Enumeration;->nextElement",
            "Ljava/util/zip/ZipEntry;->isDirectory",
            "Ljava/util/zip/ZipEntry;->getName",
            "Lcom/github/mertakdut/Content;->addEntryName",
            "Ljava/lang/String;->equals",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->newInstance",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setNamespaceAware",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setValidating",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setFeature",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setFeature",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setFeature",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->setFeature",
            "Ljavax/xml/parsers/ParserConfigurationException;->printStackTrace",
            "Ljavax/xml/parsers/DocumentBuilderFactory;->newDocumentBuilder",
            "Lcom/github/mertakdut/Content;->getEntryNames",
            "Ljava/util/List;->size",
            "Lcom/github/mertakdut/Content;->getEntryNames",
            "Ljava/util/List;->get",
            "Ljava/lang/String;->contains",
            "Ljava/util/zip/ZipFile;->getEntry",
            "Ljava/util/zip/ZipFile;->getInputStream",
            "Lcom/github/mertakdut/Reader;->getDocument",
            "Lcom/github/mertakdut/Reader;->parseContainerXml",
            "Ljava/io/IOException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/io/IOException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Ljava/lang/String;->endsWith",
            "Ljava/util/zip/ZipFile;->getEntry",
            "Ljava/util/zip/ZipFile;->getInputStream",
            "Lcom/github/mertakdut/Reader;->getDocument",
            "Lcom/github/mertakdut/Content;->getToc",
            "Lcom/github/mertakdut/Reader;->parseTocFile",
            "Ljava/io/IOException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/io/IOException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Lcom/github/mertakdut/Reader;->mergeTocElements",
            "Ljava/util/zip/ZipFile;->close",
            "Ljava/io/IOException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/io/IOException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Ljavax/xml/parsers/ParserConfigurationException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljavax/xml/parsers/ParserConfigurationException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Ljava/io/IOException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/io/IOException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Ljava/util/zip/ZipFile;->close",
            "Ljava/io/IOException;->printStackTrace",
            "Ljava/lang/StringBuilder;-><init>",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/io/IOException;->getMessage",
            "Ljava/lang/StringBuilder;->append",
            "Ljava/lang/StringBuilder;->toString",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
            "Lcom/github/mertakdut/exception/ReadingException;-><init>",
        ];
        let apk = Apk::from_path("resources/test01.apk").expect("Can't open test apk file");
        let class = apk.dex_files[0]
            .find_class_by_name("Lcom/github/mertakdut/Reader;")
            .expect("Failed to load class")
            .unwrap();
        let mut found = false;
        for method in class.methods() {
            if method.name() == "fillContent" {
                if let Some(code) = method.code() {
                    found = true;
                    for (i, target) in get_invoked_methods_names(&code, &apk.dex_files[0]).enumerate() {
                        assert_eq!(target, calls[i]);
                    }
                }
            }
        }

        assert!(
            found,
            "The test method to disassemble could not be found. Test missed!"
        );
    }
}
