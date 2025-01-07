/*++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 + Copyright (c) 2025. Xodium.
 + All rights reserved.
 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*/

const cacheName = "xodium_xbim_pwa";
const filesToCache = ["./", "./index.html", "./xbim.js", "./xbim_bg.wasm"];

/* Start the service worker and cache all the app's content */
self.addEventListener("install", e =>
    e.waitUntil(
        caches.open(cacheName).then(cache => cache.addAll(filesToCache))
    ));

/* Serve cached content when offline */
self.addEventListener("fetch", e =>
    e.respondWith(
        caches.match(e.request).then(response => response || fetch(e.request))
    ));
