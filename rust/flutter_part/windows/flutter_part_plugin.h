#ifndef FLUTTER_PLUGIN_FLUTTER_PART_PLUGIN_H_
#define FLUTTER_PLUGIN_FLUTTER_PART_PLUGIN_H_

#include <flutter/method_channel.h>
#include <flutter/plugin_registrar_windows.h>

#include <memory>

namespace flutter_part {

class FlutterPartPlugin : public flutter::Plugin {
 public:
  static void RegisterWithRegistrar(flutter::PluginRegistrarWindows *registrar);

  FlutterPartPlugin();

  virtual ~FlutterPartPlugin();

  // Disallow copy and assign.
  FlutterPartPlugin(const FlutterPartPlugin&) = delete;
  FlutterPartPlugin& operator=(const FlutterPartPlugin&) = delete;

 private:
  // Called when a method is called on this plugin's channel from Dart.
  void HandleMethodCall(
      const flutter::MethodCall<flutter::EncodableValue> &method_call,
      std::unique_ptr<flutter::MethodResult<flutter::EncodableValue>> result);
};

}  // namespace flutter_part

#endif  // FLUTTER_PLUGIN_FLUTTER_PART_PLUGIN_H_
