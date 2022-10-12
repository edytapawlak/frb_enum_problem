#include "include/flutter_part/flutter_part_plugin_c_api.h"

#include <flutter/plugin_registrar_windows.h>

#include "flutter_part_plugin.h"

void FlutterPartPluginCApiRegisterWithRegistrar(
    FlutterDesktopPluginRegistrarRef registrar) {
  flutter_part::FlutterPartPlugin::RegisterWithRegistrar(
      flutter::PluginRegistrarManager::GetInstance()
          ->GetRegistrar<flutter::PluginRegistrarWindows>(registrar));
}
