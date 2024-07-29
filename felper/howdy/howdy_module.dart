import 'package:flutter_modular/flutter_modular.dart';
import 'howdy.dart';

/// [HowdyModule] is a [Module] that provides the application's dependencies.
class HowdyModule extends Module {
  @override
  void binds(Injector i) {
    i.addLazySingleton<HowdyBloc>(() => HowdyBloc()..add(const HowdyEvent.started()));
  }

  @override
  void routes(RouteManager r) {
    r.child(
      '/',
      child: (context) => const HowdyPage(),
    );
  }
}