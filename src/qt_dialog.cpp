#include <QApplication>
#include <QDialog>
#include <QDialogButtonBox>
#include <QGuiApplication>
#include <QHBoxLayout>
#include <QIcon>
#include <QLabel>
#include <QPushButton>
#include <QString>
#include <QVBoxLayout>

namespace {
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

    QGuiApplication::setDesktopFileName(QStringLiteral("flufflinux-packagehelper"));
    application.setApplicationDisplayName(from_utf8(title));
    application.setLayoutDirection(
        right_to_left ? Qt::RightToLeft : Qt::LeftToRight);

    QDialog dialog;
    dialog.setAttribute(Qt::WA_DeleteOnClose, false);
    dialog.setWindowTitle(from_utf8(title));
    dialog.setWindowIcon(QIcon::fromTheme(QStringLiteral("dialog-information")));
    dialog.setLayoutDirection(
        right_to_left ? Qt::RightToLeft : Qt::LeftToRight);
    dialog.setMinimumWidth(520);

    auto *outer_layout = new QVBoxLayout(&dialog);
    auto *content_layout = new QHBoxLayout;
    content_layout->setSpacing(18);

    auto *icon = new QLabel(&dialog);
    icon->setPixmap(
        QIcon::fromTheme(QStringLiteral("dialog-information")).pixmap(64, 64));
    icon->setAlignment(Qt::AlignTop | Qt::AlignHCenter);
    icon->setFixedWidth(72);

    auto *text = new QLabel(from_utf8(message), &dialog);
    text->setWordWrap(true);
    text->setTextFormat(Qt::PlainText);
    text->setTextInteractionFlags(Qt::TextSelectableByMouse);
    text->setAlignment(
        (right_to_left ? Qt::AlignRight : Qt::AlignLeft) | Qt::AlignTop);
    text->setMinimumWidth(390);

    content_layout->addWidget(icon);
    content_layout->addWidget(text, 1);
    outer_layout->addLayout(content_layout);

    auto *buttons = new QDialogButtonBox(&dialog);
    auto *accept = buttons->addButton(
        from_utf8(accept_button), QDialogButtonBox::AcceptRole);
    accept->setDefault(true);
    QObject::connect(accept, &QPushButton::clicked, &dialog, &QDialog::accept);
    outer_layout->addWidget(buttons);

    return dialog.exec();
}
