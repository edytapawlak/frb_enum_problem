import 'package:flutter_part/bridge_generated.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_part/flutter_part.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';


void main() {

  test('generateText', () async {
    var text = await FlutterPart.generateText(strategy: PrefixAdder.A, basicText: 'text');
    print(text);
  });
}
