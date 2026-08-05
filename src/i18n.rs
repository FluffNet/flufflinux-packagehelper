use std::env;

pub struct Translation {
    pub title: &'static str,
    pub file_is_package: &'static str,
    pub incompatibility: &'static str,
    pub alternative: &'static str,
    pub accept: &'static str,
    pub rtl: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageFormat {
    Debian,
    Rpm,
}

impl Translation {
    pub fn message(&self, file: &str, format: PackageFormat) -> String {
        // Isolates mixed-direction filenames inside Hebrew, Arabic, and Persian text.
        let isolated_file = format!("\u{2068}{file}\u{2069}");
        let package_text = |text: &str| match format {
            PackageFormat::Debian => text.to_owned(),
            PackageFormat::Rpm => text
                .replace("Debiana", "RPM")
                .replace("Debianu", "RPM")
                .replace("Debian", "RPM"),
        };

        format!(
            "{}\n\n{}\n\n{}",
            package_text(self.file_is_package).replace("{file}", &isolated_file),
            package_text(self.incompatibility),
            self.alternative
        )
    }
}

pub fn system_locale() -> String {
    ["LC_ALL", "LANGUAGE", "LC_MESSAGES", "LANG"]
        .into_iter()
        .find_map(|name| env::var(name).ok().filter(|value| !value.is_empty()))
        .unwrap_or_else(|| "en".to_owned())
}

fn language(locale: &str) -> String {
    locale
        .split(':')
        .next()
        .unwrap_or(locale)
        .split('.')
        .next()
        .unwrap_or(locale)
        .split('@')
        .next()
        .unwrap_or(locale)
        .replace('-', "_")
        .to_ascii_lowercase()
}

pub fn translation(locale: &str) -> &'static Translation {
    let locale = language(locale);
    match locale.as_str() {
        "pt_br" => &PT_BR,
        "zh_cn" | "zh_hans" => &ZH_CN,
        "zh_tw" | "zh_hant" | "zh_hk" => &ZH_TW,
        _ => match locale.split('_').next().unwrap_or("en") {
            "he" => &HE,
            "ar" => &AR,
            "fr" => &FR,
            "ru" => &RU,
            "de" => &DE,
            "es" => &ES,
            "it" => &IT,
            "pt" => &PT,
            "nl" => &NL,
            "pl" => &PL,
            "uk" => &UK,
            "cs" => &CS,
            "sk" => &SK,
            "sv" => &SV,
            "da" => &DA,
            "nb" | "no" => &NB,
            "fi" => &FI,
            "tr" => &TR,
            "el" => &EL,
            "hu" => &HU,
            "ro" => &RO,
            "bg" => &BG,
            "ca" => &CA,
            "eu" => &EU,
            "ja" => &JA,
            "ko" => &KO,
            "zh" => &ZH_CN,
            "id" => &ID,
            "vi" => &VI,
            "fa" => &FA,
            _ => &EN,
        },
    }
}

macro_rules! tr {
    ($name:ident, $title:expr, $file:expr, $incompatible:expr, $alternative:expr, $accept:expr, $rtl:expr) => {
        static $name: Translation = Translation {
            title: $title,
            file_is_package: $file,
            incompatibility: $incompatible,
            alternative: $alternative,
            accept: $accept,
            rtl: $rtl,
        };
    };
}

tr!(
    EN,
    "Fluff Linux Package Helper",
    "The file “{file}” is a Debian package.",
    "Debian packages aren’t compatible with Fluff Linux because Fluff Linux is Arch-based and uses a different packaging system.",
    "To install software on Fluff Linux, use the Software Center (Discover) from the application launcher, or use pacman in a terminal such as Konsole.",
    "OK",
    false
);
tr!(
    HE,
    "מסייע החבילות של Fluff Linux",
    "הקובץ „{file}” הוא חבילת Debian.",
    "חבילות Debian אינן תואמות ל־Fluff Linux, מפני ש־Fluff Linux מבוססת על Arch ומשתמשת במערכת חבילות אחרת.",
    "כדי להתקין תוכנות ב־Fluff Linux, יש להשתמש במרכז התוכנה (Discover) דרך מפעיל היישומים, או ב־pacman במסוף כגון Konsole.",
    "אישור",
    true
);
tr!(
    AR,
    "مساعد حزم Fluff Linux",
    "الملف «{file}» هو حزمة Debian.",
    "حزم Debian غير متوافقة مع Fluff Linux، لأن Fluff Linux مبني على Arch ويستخدم نظام حزم مختلفًا.",
    "لتثبيت البرامج على Fluff Linux، استخدم مركز البرمجيات (Discover) من مُشغّل التطبيقات، أو استخدم pacman في طرفية مثل Konsole.",
    "موافق",
    true
);
tr!(
    FR,
    "Assistant de paquets Fluff Linux",
    "Le fichier « {file} » est un paquet Debian.",
    "Les paquets Debian ne sont pas compatibles avec Fluff Linux, car Fluff Linux repose sur Arch et utilise un autre système de paquets.",
    "Pour installer un logiciel sous Fluff Linux, utilisez la logithèque (Discover) depuis le lanceur d’applications, ou utilisez pacman dans un terminal tel que Konsole.",
    "OK",
    false
);
tr!(
    RU,
    "Помощник пакетов Fluff Linux",
    "Файл «{file}» является пакетом Debian.",
    "Пакеты Debian несовместимы с Fluff Linux, поскольку Fluff Linux основан на Arch и использует другую систему управления пакетами.",
    "Для установки программ в Fluff Linux используйте Центр приложений (Discover) из меню приложений или pacman в терминале, например Konsole.",
    "ОК",
    false
);
tr!(
    DE,
    "Fluff-Linux-Paketassistent",
    "Die Datei „{file}“ ist ein Debian-Paket.",
    "Debian-Pakete sind nicht mit Fluff Linux kompatibel, da Fluff Linux auf Arch basiert und ein anderes Paketsystem verwendet.",
    "Installieren Sie Software unter Fluff Linux über die Softwareverwaltung (Discover) im Anwendungsstarter oder mit pacman in einem Terminal wie Konsole.",
    "OK",
    false
);
tr!(
    ES,
    "Asistente de paquetes de Fluff Linux",
    "El archivo «{file}» es un paquete de Debian.",
    "Los paquetes de Debian no son compatibles con Fluff Linux porque Fluff Linux se basa en Arch y utiliza un sistema de paquetes diferente.",
    "Para instalar programas en Fluff Linux, usa el centro de software (Discover) desde el lanzador de aplicaciones o pacman en una terminal como Konsole.",
    "Aceptar",
    false
);
tr!(
    IT,
    "Assistente pacchetti di Fluff Linux",
    "Il file «{file}» è un pacchetto Debian.",
    "I pacchetti Debian non sono compatibili con Fluff Linux perché Fluff Linux è basato su Arch e usa un sistema di pacchetti diverso.",
    "Per installare software su Fluff Linux, usa il centro software (Discover) dall’avviatore delle applicazioni oppure pacman in un terminale come Konsole.",
    "OK",
    false
);
tr!(
    PT,
    "Assistente de pacotes do Fluff Linux",
    "O ficheiro «{file}» é um pacote Debian.",
    "Os pacotes Debian não são compatíveis com o Fluff Linux, pois o Fluff Linux baseia-se no Arch e utiliza um sistema de pacotes diferente.",
    "Para instalar aplicações no Fluff Linux, utilize o Centro de Software (Discover) no lançador de aplicações ou o pacman num terminal como o Konsole.",
    "OK",
    false
);
tr!(
    PT_BR,
    "Assistente de pacotes do Fluff Linux",
    "O arquivo “{file}” é um pacote Debian.",
    "Os pacotes Debian não são compatíveis com o Fluff Linux, pois o Fluff Linux é baseado no Arch e usa um sistema de pacotes diferente.",
    "Para instalar programas no Fluff Linux, use a Central de Aplicativos (Discover) pelo lançador de aplicativos ou use o pacman em um terminal como o Konsole.",
    "OK",
    false
);
tr!(
    NL,
    "Fluff Linux-pakkethulp",
    "Het bestand ‘{file}’ is een Debian-pakket.",
    "Debian-pakketten zijn niet compatibel met Fluff Linux, omdat Fluff Linux op Arch is gebaseerd en een ander pakketsysteem gebruikt.",
    "Installeer software op Fluff Linux via het softwarecentrum (Discover) in de programmastarter of met pacman in een terminal zoals Konsole.",
    "OK",
    false
);
tr!(
    PL,
    "Pomocnik pakietów Fluff Linux",
    "Plik „{file}” jest pakietem Debiana.",
    "Pakiety Debiana nie są zgodne z Fluff Linux, ponieważ Fluff Linux bazuje na Archu i używa innego systemu pakietów.",
    "Aby zainstalować oprogramowanie w Fluff Linux, użyj Centrum oprogramowania (Discover) z menu programów albo programu pacman w terminalu, takim jak Konsole.",
    "OK",
    false
);
tr!(
    UK,
    "Помічник пакунків Fluff Linux",
    "Файл «{file}» є пакунком Debian.",
    "Пакунки Debian несумісні з Fluff Linux, оскільки Fluff Linux базується на Arch і використовує іншу систему пакунків.",
    "Щоб установити програми у Fluff Linux, скористайтеся Центром програм (Discover) з меню програм або pacman у терміналі, наприклад Konsole.",
    "Гаразд",
    false
);
tr!(
    CS,
    "Pomocník balíčků Fluff Linux",
    "Soubor „{file}“ je balíček Debianu.",
    "Balíčky Debianu nejsou kompatibilní s Fluff Linuxem, protože Fluff Linux vychází z Archu a používá jiný balíčkovací systém.",
    "Software ve Fluff Linuxu instalujte pomocí Centra softwaru (Discover) ze spouštěče aplikací nebo příkazem pacman v terminálu, například Konsole.",
    "OK",
    false
);
tr!(
    SK,
    "Pomocník balíkov Fluff Linux",
    "Súbor „{file}“ je balík Debianu.",
    "Balíky Debianu nie sú kompatibilné s Fluff Linuxom, pretože Fluff Linux je založený na Archi a používa iný balíčkovací systém.",
    "Softvér vo Fluff Linuxe nainštalujte cez Centrum softvéru (Discover) zo spúšťača aplikácií alebo pomocou pacman v termináli, napríklad Konsole.",
    "OK",
    false
);
tr!(
    SV,
    "Fluff Linux pakethjälp",
    "Filen ”{file}” är ett Debian-paket.",
    "Debian-paket är inte kompatibla med Fluff Linux eftersom Fluff Linux är baserat på Arch och använder ett annat paketsystem.",
    "Installera program i Fluff Linux via programcentralen (Discover) från programstartaren eller med pacman i en terminal, till exempel Konsole.",
    "OK",
    false
);
tr!(
    DA,
    "Fluff Linux-pakkehjælper",
    "Filen “{file}” er en Debian-pakke.",
    "Debian-pakker er ikke kompatible med Fluff Linux, fordi Fluff Linux er baseret på Arch og bruger et andet pakkesystem.",
    "Installér programmer i Fluff Linux via softwarecentret (Discover) fra programstarteren eller med pacman i en terminal som Konsole.",
    "OK",
    false
);
tr!(
    NB,
    "Fluff Linux-pakkehjelper",
    "Filen «{file}» er en Debian-pakke.",
    "Debian-pakker er ikke kompatible med Fluff Linux fordi Fluff Linux er basert på Arch og bruker et annet pakkesystem.",
    "Installer programmer i Fluff Linux via programsenteret (Discover) fra programstarteren, eller med pacman i en terminal som Konsole.",
    "OK",
    false
);
tr!(
    FI,
    "Fluff Linuxin pakettiavustaja",
    "Tiedosto ”{file}” on Debian-paketti.",
    "Debian-paketit eivät ole yhteensopivia Fluff Linuxin kanssa, koska Fluff Linux perustuu Archiin ja käyttää eri paketinhallintajärjestelmää.",
    "Asenna ohjelmia Fluff Linuxiin sovelluskäynnistimen Ohjelmistokeskuksella (Discover) tai pacmanilla päätteessä, kuten Konsolessa.",
    "OK",
    false
);
tr!(
    TR,
    "Fluff Linux Paket Yardımcısı",
    "“{file}” dosyası bir Debian paketidir.",
    "Fluff Linux, Arch tabanlı olduğu ve farklı bir paket sistemi kullandığı için Debian paketleriyle uyumlu değildir.",
    "Fluff Linux’a yazılım kurmak için uygulama başlatıcısındaki Yazılım Merkezi’ni (Discover) veya Konsole gibi bir uçbirimde pacman’ı kullanın.",
    "Tamam",
    false
);
tr!(
    EL,
    "Βοηθός πακέτων Fluff Linux",
    "Το αρχείο «{file}» είναι πακέτο Debian.",
    "Τα πακέτα Debian δεν είναι συμβατά με το Fluff Linux, επειδή το Fluff Linux βασίζεται στο Arch και χρησιμοποιεί διαφορετικό σύστημα πακέτων.",
    "Για εγκατάσταση λογισμικού στο Fluff Linux, χρησιμοποιήστε το Κέντρο λογισμικού (Discover) από την εκκίνηση εφαρμογών ή το pacman σε ένα τερματικό όπως το Konsole.",
    "Εντάξει",
    false
);
tr!(
    HU,
    "Fluff Linux csomagsegéd",
    "A(z) „{file}” fájl egy Debian-csomag.",
    "A Debian-csomagok nem használhatók a Fluff Linuxon, mert a Fluff Linux Arch-alapú, és más csomagrendszert használ.",
    "Szoftver telepítéséhez használja az alkalmazásindítóból elérhető Szoftverközpontot (Discover), vagy a pacman parancsot egy terminálban, például a Konsole-ban.",
    "OK",
    false
);
tr!(
    RO,
    "Asistent de pachete Fluff Linux",
    "Fișierul „{file}” este un pachet Debian.",
    "Pachetele Debian nu sunt compatibile cu Fluff Linux, deoarece Fluff Linux se bazează pe Arch și folosește un alt sistem de pachete.",
    "Pentru a instala programe în Fluff Linux, folosește Centrul de programe (Discover) din lansatorul de aplicații sau pacman într-un terminal precum Konsole.",
    "OK",
    false
);
tr!(
    BG,
    "Помощник за пакети на Fluff Linux",
    "Файлът „{file}“ е пакет на Debian.",
    "Пакетите на Debian не са съвместими с Fluff Linux, защото Fluff Linux е базиран на Arch и използва различна пакетна система.",
    "За да инсталирате софтуер във Fluff Linux, използвайте Софтуерния център (Discover) от менюто с приложения или pacman в терминал като Konsole.",
    "Добре",
    false
);
tr!(
    CA,
    "Assistent de paquets del Fluff Linux",
    "El fitxer «{file}» és un paquet Debian.",
    "Els paquets Debian no són compatibles amb el Fluff Linux, perquè el Fluff Linux es basa en Arch i utilitza un sistema de paquets diferent.",
    "Per instal·lar programari al Fluff Linux, utilitzeu el Centre de programari (Discover) des del llançador d’aplicacions o pacman en un terminal com ara el Konsole.",
    "D’acord",
    false
);
tr!(
    EU,
    "Fluff Linux pakete-laguntzailea",
    "“{file}” fitxategia Debian pakete bat da.",
    "Debian paketeak ez dira Fluff Linuxekin bateragarriak, Fluff Linux Arch-en oinarritzen baita eta beste pakete-sistema bat erabiltzen baitu.",
    "Fluff Linuxen softwarea instalatzeko, erabili aplikazio-abiarazleko Software-zentroa (Discover) edo pacman Konsole bezalako terminal batean.",
    "Ados",
    false
);
tr!(
    JA,
    "Fluff Linux パッケージヘルパー",
    "ファイル「{file}」は Debian パッケージです。",
    "Fluff Linux は Arch をベースとしており、異なるパッケージシステムを使用しているため、Debian パッケージには対応していません。",
    "Fluff Linux にソフトウェアをインストールするには、アプリケーションランチャーからソフトウェアセンター (Discover) を使用するか、Konsole などの端末で pacman を使用してください。",
    "OK",
    false
);
tr!(
    KO,
    "Fluff Linux 패키지 도우미",
    "“{file}” 파일은 Debian 패키지입니다.",
    "Fluff Linux는 Arch 기반이며 다른 패키지 시스템을 사용하므로 Debian 패키지와 호환되지 않습니다.",
    "Fluff Linux에 소프트웨어를 설치하려면 앱 실행기의 소프트웨어 센터(Discover)를 사용하거나 Konsole과 같은 터미널에서 pacman을 사용하세요.",
    "확인",
    false
);
tr!(
    ZH_CN,
    "Fluff Linux 软件包助手",
    "文件“{file}”是 Debian 软件包。",
    "Debian 软件包与 Fluff Linux 不兼容，因为 Fluff Linux 基于 Arch，并使用不同的软件包系统。",
    "要在 Fluff Linux 上安装软件，请从应用程序启动器打开软件中心（Discover），或在 Konsole 等终端中使用 pacman。",
    "确定",
    false
);
tr!(
    ZH_TW,
    "Fluff Linux 軟體包助手",
    "檔案「{file}」是 Debian 軟體包。",
    "Debian 軟體包與 Fluff Linux 不相容，因為 Fluff Linux 以 Arch 為基礎，並使用不同的軟體包系統。",
    "若要在 Fluff Linux 安裝軟體，請從應用程式啟動器開啟軟體中心（Discover），或在 Konsole 等終端機中使用 pacman。",
    "確定",
    false
);
tr!(
    ID,
    "Pembantu Paket Fluff Linux",
    "Berkas “{file}” adalah paket Debian.",
    "Paket Debian tidak kompatibel dengan Fluff Linux karena Fluff Linux berbasis Arch dan menggunakan sistem paket yang berbeda.",
    "Untuk memasang perangkat lunak di Fluff Linux, gunakan Pusat Perangkat Lunak (Discover) dari peluncur aplikasi atau gunakan pacman di terminal seperti Konsole.",
    "Oke",
    false
);
tr!(
    VI,
    "Trợ lý gói Fluff Linux",
    "Tệp “{file}” là một gói Debian.",
    "Gói Debian không tương thích với Fluff Linux vì Fluff Linux dựa trên Arch và sử dụng một hệ thống gói khác.",
    "Để cài đặt phần mềm trên Fluff Linux, hãy dùng Trung tâm phần mềm (Discover) từ trình khởi chạy ứng dụng hoặc dùng pacman trong một trình dòng lệnh như Konsole.",
    "OK",
    false
);
tr!(
    FA,
    "دستیار بستهٔ Fluff Linux",
    "پروندهٔ «{file}» یک بستهٔ Debian است.",
    "بسته‌های Debian با Fluff Linux سازگار نیستند، زیرا Fluff Linux بر پایهٔ Arch ساخته شده و از سامانهٔ بسته‌بندی متفاوتی استفاده می‌کند.",
    "برای نصب نرم‌افزار در Fluff Linux، از مرکز نرم‌افزار (Discover) در راه‌انداز برنامه‌ها یا از pacman در پایانه‌ای مانند Konsole استفاده کنید.",
    "تأیید",
    true
);

#[cfg(test)]
mod tests {
    use super::{PackageFormat, language, translation};

    #[test]
    fn normalizes_kde_locale_names() {
        assert_eq!(language("he_IL.UTF-8"), "he_il");
        assert_eq!(language("pt-BR@latin"), "pt_br");
        assert_eq!(language("fr_CA:en_US"), "fr_ca");
    }

    #[test]
    fn enables_rtl_for_rtl_languages() {
        assert!(translation("he_IL").rtl);
        assert!(translation("ar_EG").rtl);
        assert!(translation("ar_DZ").rtl);
        assert!(translation("fa_IR").rtl);
        assert!(!translation("fr_FR").rtl);
    }

    #[test]
    fn localizes_rpm_without_translating_product_names() {
        for locale in ["he_IL", "fr_FR", "ar_EG", "ru_RU", "pl_PL", "fi_FI"] {
            let message = translation(locale).message("example.rpm", PackageFormat::Rpm);
            assert!(message.contains("RPM"));
            assert!(!message.contains("Debian"));
            assert!(message.contains("Discover"));
        }
    }
}
