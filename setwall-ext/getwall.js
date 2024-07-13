browser.menus.create({
    id: "image_clicked_by_setwall",
    title: "set as wall",
    contexts: ["image"],
    onclick: (info, _) => {
        console.log(info);
        let asock = new WebSocket("ws://127.0.0.1:7878", "rust-websocket");
        asock.onopen = (_) => {
            asock.send(`url by setwall:` + info.srcUrl);
        };
    }
});

browser.runtime.onMessage.addListener(
    (data, _) => {
        if (data === "visited") {
            // 
            browser.menus.create({
                id: "wallhaven_link_clicked_by_setwall",
                title: "setwall by link",
                contexts: ["link"],
                onclick: (info, _) => {
                    console.log(info);
                    let asock = new WebSocket("ws://127.0.0.1:7878", "rust-websocket");
                    asock.onopen = (_) => {
                        asock.send(`wallhaven url:` + info.linkUrl);
                    };
                }
            });
            return Promise.resolve('done');
        }
    }
);
