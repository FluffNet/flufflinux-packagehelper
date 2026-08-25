#include <QApplication>
#include <QDialog>
#include <QDialogButtonBox>
#include <QFontMetrics>
#include <QGuiApplication>
#include <QHBoxLayout>
#include <QIcon>
#include <QLabel>
#include <QPushButton>
#include <QScreen>
#include <QString>
#include <QStringList>
#include <QVBoxLayout>
#include <QWidget>

namespace {
// Convert Rust text into Qt text
QString from_utf8(const char *text) {
    return QString::fromUtf8(text == nullptr ? "" : text);
}

}

extern "C" int flufflinux_show_information_dialog(
    const char *title,
    const char *message,
    const char *accept_button,
    bool right_to_left) {
    int argc = 1;
    char application_name[] = "flufflinux-packagehelper";
    char *argv[] = {application_name, nullptr};
    QApplication application(argc, argv);

    // Use the desktop identity and the active KDE theme
    QGuiApplication::setDesktopFileName(QStringLiteral("flufflinux-packagehelper"));
    application.setApplicationDisplayName(from_utf8(title));
    const QIcon application_icon =
        QIcon::fromTheme(QStringLiteral("package"));
    application.setWindowIcon(application_icon);
    application.setLayoutDirection(
        right_to_left ? Qt::RightToLeft : Qt::LeftToRight);

    // Keep only the title and close controls
    QDialog dialog;
    dialog.setAttribute(Qt::WA_DeleteOnClose, false);
    dialog.setWindowFlags(
        Qt::Dialog |
        Qt::CustomizeWindowHint |
        Qt::WindowTitleHint |
        Qt::WindowCloseButtonHint |
        Qt::WindowStaysOnTopHint);
    dialog.setWindowTitle(from_utf8(title));
    dialog.setWindowIcon(application_icon);
    dialog.setLayoutDirection(
        right_to_left ? Qt::RightToLeft : Qt::LeftToRight);

    // Place the KDE information icon beside the translated text
    auto *outer_layout = new QVBoxLayout(&dialog);
    auto *content_layout = new QHBoxLayout;
    content_layout->setSpacing(18);

    auto *icon = new QLabel(&dialog);
    icon->setPixmap(
        QIcon::fromTheme(QStringLiteral("dialog-information")).pixmap(64, 64));
    icon->setAlignment(Qt::AlignTop | Qt::AlignHCenter);
    icon->setFixedWidth(72);

    // Measure translated text before fixing the dialog size
    const QRect available_screen =
        application.primaryScreen()->availableGeometry();
    const int text_width =
        qMax(430, qMin(640, available_screen.width() - 180));

    const QString plain_message = from_utf8(message);
    QWidget *text_area = nullptr;

    if (right_to_left) {
        auto *paragraphs = new QWidget(&dialog);
        auto *paragraph_layout = new QVBoxLayout(paragraphs);
        paragraph_layout->setContentsMargins(0, 0, 0, 0);
        paragraph_layout->setSpacing(0);

        const QStringList lines = plain_message.split(QLatin1Char('\n'));
        const QFontMetrics text_metrics(dialog.font());
        const auto add_line = [&](int index, Qt::LayoutDirection direction) {
            QString line = lines.value(index);

            auto *label = new QLabel(line, paragraphs);
            label->setWordWrap(true);
            label->setTextFormat(Qt::PlainText);
            label->setTextInteractionFlags(Qt::TextSelectableByMouse);
            label->setLayoutDirection(direction);
            label->setAlignment(
                Qt::AlignRight | Qt::AlignTop | Qt::AlignAbsolute);

            const QRect measured_line = text_metrics.boundingRect(
                QRect(0, 0, text_width, QWIDGETSIZE_MAX),
                Qt::TextWordWrap | Qt::TextWrapAnywhere,
                line);
            label->setFixedSize(text_width, measured_line.height() + 2);
            paragraph_layout->addWidget(label);
        };

        add_line(0, Qt::RightToLeft);
        paragraph_layout->addSpacing(text_metrics.lineSpacing());
        add_line(2, Qt::RightToLeft);
        paragraph_layout->addSpacing(text_metrics.lineSpacing());
        add_line(4, Qt::RightToLeft);
        paragraphs->setFixedSize(text_width, paragraph_layout->sizeHint().height());
        text_area = paragraphs;
    } else {
        auto *text = new QLabel(plain_message, &dialog);
        text->setWordWrap(true);
        text->setTextFormat(Qt::PlainText);
        text->setTextInteractionFlags(Qt::TextSelectableByMouse);
        text->setAlignment(Qt::AlignLeft | Qt::AlignTop);

        const QFontMetrics text_metrics(text->font());
        const QRect measured_text = text_metrics.boundingRect(
            QRect(0, 0, text_width, QWIDGETSIZE_MAX),
            Qt::TextWordWrap | Qt::TextWrapAnywhere,
            plain_message);
        text->setFixedSize(text_width, measured_text.height() + 4);
        text_area = text;
    }

    content_layout->addWidget(icon);
    content_layout->addWidget(text_area, 1);
    outer_layout->addLayout(content_layout);

    // Use one translated confirmation button
    auto *buttons = new QDialogButtonBox(&dialog);
    auto *accept = buttons->addButton(
        from_utf8(accept_button), QDialogButtonBox::AcceptRole);
    accept->setDefault(true);
    QObject::connect(accept, &QPushButton::clicked, &dialog, &QDialog::accept);
    outer_layout->addSpacing(4);
    outer_layout->addWidget(buttons);

    // Lock the complete measured layout while keeping the window movable
    dialog.adjustSize();
    dialog.setFixedSize(dialog.size());

    return dialog.exec();
}
