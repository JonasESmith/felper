import 'package:bloc/bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'mods_event.dart';
part 'mods_state.dart';
part 'mods_bloc.freezed.dart';

class ModsBloc extends Bloc<ModsEvent, ModsState> {
  ModsBloc() : super(const ModsState.initial()) {
    on<ModsEvent>((event, emit) {
      // TODO: implement event handler
    });
  }
}
