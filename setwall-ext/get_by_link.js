function msg_to_ext(msg, on_response) {
    browser.runtime.sendMessage("hem1t@firefoxaddon.com", msg).then(on_response);
}

msg_to_ext("visited", () => {});
