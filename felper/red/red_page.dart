import 'package:flutter/material.dart';
import 'package:flutter_modular/flutter_modular.dart';

/// [RedPage] the display page for this feature
class RedPage extends StatelessWidget {
  /// [RedPage] constructor.
  const RedPage({super.key});

  /// [routeName] the route name for this page
  static const routeName = '/red';

  /// our route, this should generally use the modular route, and
  /// our basic route callable item
  static void route() {
    Modular.to.pushNamed(routeName);
  }

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Text(
        routeName,
      ),
    );
  }
}