import SwiftRs
import Tauri
import UIKit
import WebKit

class PingArgs: Decodable {
  let value: String?
}

class ExamplePlugin: Plugin {
  @objc public func ping(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(PingArgs.self)
    invoke.resolve(["value": args.value ?? ""])
  }
  @objc open override func load(webview: WKWebView) {
    guard let rootVC = UIApplication.shared.keyWindow?.rootViewController else { return }

    rootVC.edgesForExtendedLayout = .all
    rootVC.extendedLayoutIncludesOpaqueBars = true

    webview.backgroundColor = .clear
    webview.scrollView.backgroundColor = .clear
    webview.scrollView.contentInsetAdjustmentBehavior = .never

    webview.autoresizingMask = [.flexibleWidth, .flexibleHeight]
    webview.frame = rootVC.view.bounds

    UIApplication.shared.keyWindow?.backgroundColor = .white
  }
}

@_cdecl("init_plugin_ios_fs")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
