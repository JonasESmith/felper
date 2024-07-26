import 'package:flutter_modular/flutter_modular.dart';
import 'mods.dart';

/// [ModsModule] is a [Module] that provides the application's dependencies.
class ModsModule extends Module {
  @override
  void binds(Injector i) {
    i.addLazySingleton<ModsBloc>(() => ModsBloc()..add(const ModsEvent.started()));
  }

  @override
  void routes(RouteManager r) {
    r.child(
      '/',
      child: (context) => const ModsPage(),
    );
  }
}