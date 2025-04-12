#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// Описывает программный пакет.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Возвращает этот пакет в виде зависимости.
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: format!("={}", self.version),
        }
    }
}

/// Компилятор пакета (строитель).
struct PackageBuilder(Package);

impl PackageBuilder {
    /// Создаёт новый пакет с заданным именем, остальное пока пустое.
    fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: String::new(),
            authors: Vec::new(),
            dependencies: Vec::new(),
            language: None,
        })
    }

    /// Задает версию пакета.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Задает авторов пакета.
    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    /// Добавляет зависимость.
    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Задает язык программирования.
    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    /// Завершает сборку и возвращает готовый объект Package.
    fn build(self) -> Package {
        self.0
    }
}

fn main() {
    // Создаём пакет base64 версии 0.13 без зависимостей и авторов
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");

    // Создаём пакет log версии 0.4 на языке Rust
    let log = PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");

    // Создаём пакет serde версии 4.0 с автором, зависящий от base64 и log
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version("4.0")
        .dependency(base64.as_dependency())  // Добавляем зависимость от base64
        .dependency(log.as_dependency())     // Добавляем зависимость от log
        .build();

    println!("serde: {serde:?}");
}
