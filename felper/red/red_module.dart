import 'package:flutter_modular/flutter_modular.dart';
import 'red.dart';

/// [RedModule] is a [Module] that provides the application's dependencies.
class RedModule extends Module {
  @override
  void binds(Injector i) {
    i.addLazySingleton<RedBloc>(() => RedBloc()..add(const RedEvent.started()));
  }

  @override
  void routes(RouteManager r) {
    r.child(
      '/',
      child: (context) => const RedPage(),
    );
  }
}