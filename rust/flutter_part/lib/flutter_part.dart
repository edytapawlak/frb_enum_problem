

import 'dart:ffi';
import 'dart:io';

import 'bridge_generated.dart';

class FlutterPart {
  static late final dylib = DynamicLibrary.open(
      Platform.script.resolve("windows/rust.dll").toFilePath());
  static late final api = RustImpl(dylib);


  static Future<String> generateText(
      {required PrefixAdder strategy, required String basicText, dynamic hint}) async{
    return await api.generateText(strategy: strategy, basicText: basicText);
  }
}
