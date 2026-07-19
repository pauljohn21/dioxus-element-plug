#!/usr/bin/env python3
"""SPA-friendly HTTP server for Dioxus documentation site.

Serves static files from the build output directory.
For SPA routing, serves index.html for any non-file route.
"""

import http.server
import os
import sys

PORT = 8080
BUILD_DIR = os.path.expanduser(
    "~/.cargo/target/dx/dioxus-element-plug-docs/release/web/public"
)


class SPAHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=BUILD_DIR, **kwargs)

    def send_head(self):
        path = self.translate_path(self.path)
        # If the path exists as a file, serve it
        if os.path.isfile(path):
            return super().send_head()
        # Otherwise, serve index.html for SPA routing
        self.path = "/index.html"
        return super().send_head()


if __name__ == "__main__":
    httpd = http.server.HTTPServer(("0.0.0.0", PORT), SPAHandler)
    print(f"Serving Dioxus docs at http://localhost:{PORT}")
    print(f"Build directory: {BUILD_DIR}")
    httpd.serve_forever()