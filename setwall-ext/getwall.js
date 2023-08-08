browser.menus.create({
    id: "image_clicked_by_setwall",
    title: "setwall",
    contexts: ["image"],
    onclick: (info, tab) => {
        let asock = new WebSocket("ws://127.0.0.1:7878", "rust-websocket");
        console.log(info);
        asock.onopen = (event) => {
            asock.send(`url by setwall ` + info.srcUrl);
        };
    }
});

browser.runtime.onMessage.addListener(
    (data, sender) => {
        if (data === "visited") {
            // 
            browser.menus.create({
                id: "wallhaven_link_clicked_by_setwall",
                title: "setwall by link",
                contexts: ["link"],
                onclick: (info, tab) => {
                    let asock = new WebSocket("ws://127.0.0.1:7878", "rust-websocket");
                    console.log(info);
                    asock.onopen = (event) => {
                        asock.send(`url by setwall ` + info.linkUrl);
                    };
                }
            });
            return Promise.resolve('done');
        }
    }
);
