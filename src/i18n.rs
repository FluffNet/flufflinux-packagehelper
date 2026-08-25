use std::env;
use std::path::Path;

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
        // Keep mixed direction file names readable in right to left text
        let file_name = Path::new(file)
            .file_name()
            .map(|name| name.to_string_lossy())
            .unwrap_or_else(|| file.into());
        let isolated_file_name = format!("\u{2066}{file_name}\u{2069}");
        let package_text = |text: &str| match format {
            PackageFormat::Debian => text.to_owned(),
            PackageFormat::Rpm => text
                .replace("Debiana", "RPM")
                .replace("Debianu", "RPM")
                .replace("Debian", "RPM")
                .replace("a RPM package", "an RPM package"),
        };

        if self.rtl {
            format!(
                "\u{200f}{}\n\n\u{200f}{}\n\n\u{200f}{}",
                package_text(self.file_is_package).replace("{file}", &isolated_file_name),
                package_text(self.incompatibility),
                self.alternative
            )
        } else {
            format!(
                "{}\n\n{}\n\n{}",
                package_text(self.file_is_package).replace("{file}", &isolated_file_name),
                package_text(self.incompatibility),
                self.alternative
            )
        }
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
            "et" => &ET,
            "ga" => &GA,
            "hr" => &HR,
            "it" => &IT,
            "lt" => &LT,
            "lv" => &LV,
            "mt" => &MT,
            "pt" => &PT,
            "nl" => &NL,
            "pl" => &PL,
            "uk" => &UK,
            "cs" => &CS,
            "sk" => &SK,
            "sl" => &SL,
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
    "Debian packages aren’t compatible with Fluff Linux because Fluff Linux is Arch‑based and uses a different packaging system.",
    "Use the App Center (Discover) or search online to install a version compatible with Fluff Linux.",
    "OK",
    false
);
tr!(
    HE,
    "מסייע היישומים של Fluff Linux",
    "{file} הוא קובץ חבילה מסוג Debian.",
    "קבצים מסוג זה אינם תואמים ל Fluff Linux מכיוון ש Fluff Linux מבוססת על Arch ומשתמשת במערכת חבילות אחרת.",
    "ניתן לחפש את האפליקציה הרצויה ב״מרכז האפליקציות״ או לחפש באינטרנט גירסה אחרת שתואמת ל Fluff Linux.",
    "אישור",
    true
);
tr!(
    AR,
    "مساعد حزم Fluff Linux",
    "{file} هو ملف حزمة من نوع Debian.",
    "ملفات من هذا النوع غير متوافقة مع Fluff Linux لأن Fluff Linux مبني على Arch ويستخدم نظام حزم مختلفًا.",
    "يمكن البحث عن التطبيق المطلوب في «مركز التطبيقات» أو البحث عبر الإنترنت عن إصدار آخر متوافق مع Fluff Linux.",
    "موافق",
    true
);
tr!(
    FR,
    "Assistant de paquets Fluff Linux",
    "Le fichier « {file} » est un paquet Debian.",
    "Les paquets Debian ne sont pas compatibles avec Fluff Linux, car Fluff Linux repose sur Arch et utilise un autre système de paquets.",
    "Utilisez App Center (Discover) ou recherchez en ligne une version compatible avec Fluff Linux à installer.",
    "OK",
    false
);
tr!(
    RU,
    "Помощник пакетов Fluff Linux",
    "Файл «{file}» является пакетом Debian.",
    "Пакеты Debian несовместимы с Fluff Linux, поскольку Fluff Linux основан на Arch и использует другую систему управления пакетами.",
    "Используйте App Center (Discover) или найдите в интернете и установите версию, совместимую с Fluff Linux.",
    "ОК",
    false
);
tr!(
    DE,
    "Fluff-Linux-Paketassistent",
    "Die Datei „{file}“ ist ein Debian-Paket.",
    "Debian-Pakete sind nicht mit Fluff Linux kompatibel, da Fluff Linux auf Arch basiert und ein anderes Paketsystem verwendet.",
    "Verwenden Sie das App Center (Discover) oder suchen Sie online nach einer mit Fluff Linux kompatiblen Version, die Sie installieren können.",
    "OK",
    false
);
tr!(
    ES,
    "Asistente de paquetes de Fluff Linux",
    "El archivo «{file}» es un paquete de Debian.",
    "Los paquetes de Debian no son compatibles con Fluff Linux porque Fluff Linux se basa en Arch y utiliza un sistema de paquetes diferente.",
    "Usa App Center (Discover) o busca en Internet una versión compatible con Fluff Linux para instalarla.",
    "Aceptar",
    false
);
tr!(
    ET,
    "Fluff Linuxi paketiabiline",
    "Fail „{file}” on pakett vormingus Debian.",
    "Vormingu Debian paketid ei ühildu Fluff Linuxiga, sest Fluff Linux põhineb Archil ja kasutab teistsugust paketisüsteemi.",
    "Kasutage rakendust App Center (Discover) või otsige veebist installimiseks Fluff Linuxiga ühilduvat versiooni.",
    "Sobib",
    false
);
tr!(
    GA,
    "Cúntóir Pacáistí Fluff Linux",
    "Is pacáiste Debian é an comhad “{file}”.",
    "Níl pacáistí Debian comhoiriúnach le Fluff Linux mar tá Fluff Linux bunaithe ar Arch agus úsáideann sé córas pacáistí eile.",
    "Bain úsáid as App Center (Discover) nó cuardaigh ar líne chun leagan atá comhoiriúnach le Fluff Linux a shuiteáil.",
    "OK",
    false
);
tr!(
    HR,
    "Pomoćnik za pakete Fluff Linuxa",
    "Datoteka „{file}” paket je formata Debian.",
    "Paketi formata Debian nisu kompatibilni s Fluff Linuxom jer se Fluff Linux temelji na Archu i koristi drukčiji sustav paketa.",
    "Upotrijebite App Center (Discover) ili na internetu potražite i instalirajte verziju kompatibilnu s Fluff Linuxom.",
    "U redu",
    false
);
tr!(
    IT,
    "Assistente pacchetti di Fluff Linux",
    "Il file «{file}» è un pacchetto Debian.",
    "I pacchetti Debian non sono compatibili con Fluff Linux perché Fluff Linux è basato su Arch e usa un sistema di pacchetti diverso.",
    "Usa App Center (Discover) oppure cerca online una versione compatibile con Fluff Linux da installare.",
    "OK",
    false
);
tr!(
    LT,
    "Fluff Linux paketų pagalbininkas",
    "Failas „{file}“ yra Debian paketas.",
    "Debian paketai nesuderinami su Fluff Linux, nes Fluff Linux pagrįsta Arch ir naudoja kitą paketų sistemą.",
    "Naudokite App Center (Discover) arba internete raskite ir įdiekite su Fluff Linux suderinamą versiją.",
    "Gerai",
    false
);
tr!(
    LV,
    "Fluff Linux pakotņu palīgs",
    "Fails “{file}” ir Debian pakotne.",
    "Debian pakotnes nav saderīgas ar Fluff Linux, jo Fluff Linux pamatā ir Arch un tā izmanto citu pakotņu sistēmu.",
    "Izmantojiet App Center (Discover) vai meklējiet internetā instalējamu ar Fluff Linux saderīgu versiju.",
    "Labi",
    false
);
tr!(
    MT,
    "Assistent tal-Pakketti ta’ Fluff Linux",
    "Il-fajl “{file}” huwa pakkett Debian.",
    "Il-pakketti Debian mhumiex kompatibbli ma’ Fluff Linux għax Fluff Linux huwa bbażat fuq Arch u juża sistema ta’ pakketti differenti.",
    "Uża App Center (Discover) jew fittex online biex tinstalla verżjoni kompatibbli ma’ Fluff Linux.",
    "OK",
    false
);
tr!(
    PT,
    "Assistente de pacotes do Fluff Linux",
    "O ficheiro «{file}» é um pacote Debian.",
    "Os pacotes Debian não são compatíveis com o Fluff Linux, pois o Fluff Linux baseia-se no Arch e utiliza um sistema de pacotes diferente.",
    "Utilize a App Center (Discover) ou procure online uma versão compatível com o Fluff Linux para instalar.",
    "OK",
    false
);
tr!(
    PT_BR,
    "Assistente de pacotes do Fluff Linux",
    "O arquivo “{file}” é um pacote Debian.",
    "Os pacotes Debian não são compatíveis com o Fluff Linux, pois o Fluff Linux é baseado no Arch e usa um sistema de pacotes diferente.",
    "Use o App Center (Discover) ou procure online uma versão compatível com o Fluff Linux para instalar.",
    "OK",
    false
);
tr!(
    NL,
    "Fluff Linux-pakkethulp",
    "Het bestand ‘{file}’ is een Debian-pakket.",
    "Debian-pakketten zijn niet compatibel met Fluff Linux, omdat Fluff Linux op Arch is gebaseerd en een ander pakketsysteem gebruikt.",
    "Gebruik App Center (Discover) of zoek online naar een versie die compatibel is met Fluff Linux om te installeren.",
    "OK",
    false
);
tr!(
    PL,
    "Pomocnik pakietów Fluff Linux",
    "Plik „{file}” jest pakietem Debiana.",
    "Pakiety Debiana nie są zgodne z Fluff Linux, ponieważ Fluff Linux bazuje na Archu i używa innego systemu pakietów.",
    "Użyj App Center (Discover) albo znajdź w internecie i zainstaluj wersję zgodną z Fluff Linux.",
    "OK",
    false
);
tr!(
    UK,
    "Помічник пакунків Fluff Linux",
    "Файл «{file}» є пакунком Debian.",
    "Пакунки Debian несумісні з Fluff Linux, оскільки Fluff Linux базується на Arch і використовує іншу систему пакунків.",
    "Скористайтеся App Center (Discover) або знайдіть в інтернеті та встановіть версію, сумісну з Fluff Linux.",
    "Гаразд",
    false
);
tr!(
    CS,
    "Pomocník balíčků Fluff Linux",
    "Soubor „{file}“ je balíček Debianu.",
    "Balíčky Debianu nejsou kompatibilní s Fluff Linuxem, protože Fluff Linux vychází z Archu a používá jiný balíčkovací systém.",
    "Použijte App Center (Discover) nebo online vyhledejte a nainstalujte verzi kompatibilní s Fluff Linuxem.",
    "OK",
    false
);
tr!(
    SK,
    "Pomocník balíkov Fluff Linux",
    "Súbor „{file}“ je balík Debianu.",
    "Balíky Debianu nie sú kompatibilné s Fluff Linuxom, pretože Fluff Linux je založený na Archi a používa iný balíčkovací systém.",
    "Použite App Center (Discover) alebo online vyhľadajte a nainštalujte verziu kompatibilnú s Fluff Linuxom.",
    "OK",
    false
);
tr!(
    SL,
    "Pomočnik za pakete Fluff Linux",
    "Datoteka »{file}« je paket Debian.",
    "Paketi Debian niso združljivi s Fluff Linuxom, ker Fluff Linux temelji na Archu in uporablja drugačen paketni sistem.",
    "Uporabite App Center (Discover) ali v spletu poiščite in namestite različico, združljivo s Fluff Linuxom.",
    "V redu",
    false
);
tr!(
    SV,
    "Fluff Linux pakethjälp",
    "Filen ”{file}” är ett Debian-paket.",
    "Debian-paket är inte kompatibla med Fluff Linux eftersom Fluff Linux är baserat på Arch och använder ett annat paketsystem.",
    "Använd App Center (Discover) eller sök på nätet efter en version som är kompatibel med Fluff Linux att installera.",
    "OK",
    false
);
tr!(
    DA,
    "Fluff Linux-pakkehjælper",
    "Filen “{file}” er en Debian-pakke.",
    "Debian-pakker er ikke kompatible med Fluff Linux, fordi Fluff Linux er baseret på Arch og bruger et andet pakkesystem.",
    "Brug App Center (Discover), eller søg online efter en version, der er kompatibel med Fluff Linux, og installer den.",
    "OK",
    false
);
tr!(
    NB,
    "Fluff Linux-pakkehjelper",
    "Filen «{file}» er en Debian-pakke.",
    "Debian-pakker er ikke kompatible med Fluff Linux fordi Fluff Linux er basert på Arch og bruker et annet pakkesystem.",
    "Bruk App Center (Discover), eller søk på nettet etter en versjon som er kompatibel med Fluff Linux, og installer den.",
    "OK",
    false
);
tr!(
    FI,
    "Fluff Linuxin pakettiavustaja",
    "Tiedosto ”{file}” on Debian-paketti.",
    "Debian-paketit eivät ole yhteensopivia Fluff Linuxin kanssa, koska Fluff Linux perustuu Archiin ja käyttää eri paketinhallintajärjestelmää.",
    "Käytä sovellusta App Center (Discover) tai etsi verkosta asennettava Fluff Linuxin kanssa yhteensopiva versio.",
    "OK",
    false
);
tr!(
    TR,
    "Fluff Linux Paket Yardımcısı",
    "“{file}” dosyası bir Debian paketidir.",
    "Fluff Linux, Arch tabanlı olduğu ve farklı bir paket sistemi kullandığı için Debian paketleriyle uyumlu değildir.",
    "App Center (Discover) uygulamasını kullanın veya yüklemek için Fluff Linux ile uyumlu bir sürümü çevrimiçi arayın.",
    "Tamam",
    false
);
tr!(
    EL,
    "Βοηθός πακέτων Fluff Linux",
    "Το αρχείο «{file}» είναι πακέτο Debian.",
    "Τα πακέτα Debian δεν είναι συμβατά με το Fluff Linux, επειδή το Fluff Linux βασίζεται στο Arch και χρησιμοποιεί διαφορετικό σύστημα πακέτων.",
    "Χρησιμοποιήστε το App Center (Discover) ή αναζητήστε στο διαδίκτυο και εγκαταστήστε μια έκδοση συμβατή με το Fluff Linux.",
    "Εντάξει",
    false
);
tr!(
    HU,
    "Fluff Linux csomagsegéd",
    "A(z) „{file}” fájl egy Debian-csomag.",
    "A Debian-csomagok nem használhatók a Fluff Linuxon, mert a Fluff Linux Arch‑alapú, és más csomagrendszert használ.",
    "Használja az App Center (Discover) alkalmazást, vagy keressen az interneten egy telepíthető, Fluff Linuxszal kompatibilis verziót.",
    "OK",
    false
);
tr!(
    RO,
    "Asistent de pachete Fluff Linux",
    "Fișierul „{file}” este un pachet Debian.",
    "Pachetele Debian nu sunt compatibile cu Fluff Linux, deoarece Fluff Linux se bazează pe Arch și folosește un alt sistem de pachete.",
    "Folosește App Center (Discover) sau caută online și instalează o versiune compatibilă cu Fluff Linux.",
    "OK",
    false
);
tr!(
    BG,
    "Помощник за пакети на Fluff Linux",
    "Файлът „{file}“ е пакет на Debian.",
    "Пакетите на Debian не са съвместими с Fluff Linux, защото Fluff Linux е базиран на Arch и използва различна пакетна система.",
    "Използвайте App Center (Discover) или намерете онлайн и инсталирайте версия, съвместима с Fluff Linux.",
    "Добре",
    false
);
tr!(
    CA,
    "Assistent de paquets del Fluff Linux",
    "El fitxer «{file}» és un paquet Debian.",
    "Els paquets Debian no són compatibles amb el Fluff Linux, perquè el Fluff Linux es basa en Arch i utilitza un sistema de paquets diferent.",
    "Utilitzeu App Center (Discover) o cerqueu en línia una versió compatible amb el Fluff Linux per instal·lar-la.",
    "D’acord",
    false
);
tr!(
    EU,
    "Fluff Linux pakete-laguntzailea",
    "“{file}” fitxategia Debian pakete bat da.",
    "Debian paketeak ez dira Fluff Linuxekin bateragarriak, Fluff Linux Arch‑en oinarritzen baita eta beste pakete-sistema bat erabiltzen baitu.",
    "Erabili App Center (Discover), edo bilatu Interneten Fluff Linuxekin bateragarria den bertsio bat instalatzeko.",
    "Ados",
    false
);
tr!(
    JA,
    "Fluff Linux パッケージヘルパー",
    "ファイル「{file}」は Debian パッケージです。",
    "Fluff Linux は Arch をベースとしており、異なるパッケージシステムを使用しているため、Debian パッケージには対応していません。",
    "App Center (Discover) を使用するか、Fluff Linux に対応したインストール可能なバージョンをオンラインで探してください。",
    "OK",
    false
);
tr!(
    KO,
    "Fluff Linux 패키지 도우미",
    "“{file}” 파일은 Debian 패키지입니다.",
    "Fluff Linux는 Arch 기반이며 다른 패키지 시스템을 사용하므로 Debian 패키지와 호환되지 않습니다.",
    "App Center (Discover)를 사용하거나 Fluff Linux와 호환되는 버전을 온라인에서 찾아 설치하세요.",
    "확인",
    false
);
tr!(
    ZH_CN,
    "Fluff Linux 软件包助手",
    "文件“{file}”是 Debian 软件包。",
    "Debian 软件包与 Fluff Linux 不兼容，因为 Fluff Linux 基于 Arch，并使用不同的软件包系统。",
    "请使用 App Center (Discover)，或在线查找并安装与 Fluff Linux 兼容的版本。",
    "确定",
    false
);
tr!(
    ZH_TW,
    "Fluff Linux 軟體包助手",
    "檔案「{file}」是 Debian 軟體包。",
    "Debian 軟體包與 Fluff Linux 不相容，因為 Fluff Linux 以 Arch 為基礎，並使用不同的軟體包系統。",
    "請使用 App Center (Discover)，或在線上尋找並安裝與 Fluff Linux 相容的版本。",
    "確定",
    false
);
tr!(
    ID,
    "Pembantu Paket Fluff Linux",
    "Berkas “{file}” adalah paket Debian.",
    "Paket Debian tidak kompatibel dengan Fluff Linux karena Fluff Linux berbasis Arch dan menggunakan sistem paket yang berbeda.",
    "Gunakan App Center (Discover) atau cari dan instal versi yang kompatibel dengan Fluff Linux secara daring.",
    "Oke",
    false
);
tr!(
    VI,
    "Trợ lý gói Fluff Linux",
    "Tệp “{file}” là một gói Debian.",
    "Gói Debian không tương thích với Fluff Linux vì Fluff Linux dựa trên Arch và sử dụng một hệ thống gói khác.",
    "Hãy dùng App Center (Discover) hoặc tìm và cài đặt trên mạng một phiên bản tương thích với Fluff Linux.",
    "OK",
    false
);
tr!(
    FA,
    "دستیار بستهٔ Fluff Linux",
    "{file} یک پروندهٔ بسته از نوع Debian است.",
    "پرونده‌های این نوع با Fluff Linux سازگار نیستند زیرا Fluff Linux بر پایهٔ Arch ساخته شده و از سامانهٔ بسته‌بندی متفاوتی استفاده می‌کند.",
    "می‌توانید برنامهٔ مورد نظر را در «مرکز برنامه‌ها» جستجو کنید یا در اینترنت نسخهٔ دیگری را که با Fluff Linux سازگار است پیدا کنید.",
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
    fn uses_the_requested_english_installation_guidance() {
        assert_eq!(
            translation("en_US").alternative,
            "Use the App Center (Discover) or search online to install a version compatible with Fluff Linux."
        );
    }

    #[test]
    fn displays_only_the_file_name() {
        let message =
            translation("en_US").message("/home/user/Downloads/example.deb", PackageFormat::Debian);

        assert!(message.contains("“\u{2066}example.deb\u{2069}”"));
        assert!(!message.contains("/home/user/Downloads/"));
        assert!(message.contains("Arch‑based"));
        assert!(!message.contains("Arch-based"));
    }

    #[test]
    fn localizes_rpm_without_translating_product_names() {
        for locale in ["fr_FR", "ru_RU", "pl_PL", "fi_FI"] {
            let message = translation(locale).message("example.rpm", PackageFormat::Rpm);
            assert!(message.contains("RPM"));
            assert!(!message.contains("Debian"));
            assert!(message.contains("App Center (Discover)"));
            assert!(message.contains("Discover"));
            assert!(!message.contains("pacman"));
        }

        assert!(
            translation("en_US")
                .message("example.rpm", PackageFormat::Rpm)
                .contains("is an RPM package")
        );
    }

    #[test]
    fn formats_right_to_left_messages() {
        for (locale, package_file_wording) in [
            ("he_IL", "הוא קובץ חבילה מסוג"),
            ("ar_EG", "هو ملف حزمة من نوع"),
            ("fa_IR", "یک پروندهٔ بسته از نوع"),
        ] {
            let message = translation(locale).message("/tmp/example.rpm", PackageFormat::Rpm);
            let lines: Vec<_> = message.lines().collect();

            assert_eq!(lines.len(), 5);
            assert!(lines[0].starts_with('\u{200f}'));
            assert!(lines[0].contains("\u{2066}example.rpm\u{2069}"));
            assert!(!lines[0].contains("/tmp/"));
            assert!(lines[0].contains("RPM"));
            assert!(lines[0].contains(package_file_wording));
            assert!(lines[0].ends_with('.'));
            assert_eq!(lines[1], "");
            assert!(lines[2].starts_with('\u{200f}'));
            assert!(lines[2].ends_with('.'));
            assert_eq!(lines[3], "");
            assert!(lines[4].starts_with('\u{200f}'));
            assert!(lines[4].ends_with('.'));
            assert!(!message.contains("Discover"));
            assert!(!message.contains("App Center"));
        }
    }

    #[test]
    fn supports_every_flufflinux_update_locale() {
        for locale in [
            "ar", "bg", "cs", "da", "de", "el", "es", "et", "fi", "fr", "ga", "he", "hr", "hu",
            "it", "ja", "lt", "lv", "mt", "nl", "pl", "pt", "ro", "ru", "sk", "sl", "sv",
        ] {
            let debian = translation(locale).message("example.deb", PackageFormat::Debian);
            let rpm = translation(locale).message("example.rpm", PackageFormat::Rpm);

            assert!(
                debian.contains("Debian"),
                "missing Debian translation for {locale}"
            );
            assert!(rpm.contains("RPM"), "missing RPM translation for {locale}");
            if translation(locale).rtl {
                assert!(!debian.contains("Discover"));
                assert!(!rpm.contains("Discover"));
            } else {
                assert!(debian.contains("App Center (Discover)"));
                assert!(rpm.contains("App Center (Discover)"));
            }
        }
    }
}
