import 'package:bloc/bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'red_event.dart';
part 'red_state.dart';
part 'red_bloc.freezed.dart';

class RedBloc extends Bloc<RedEvent, RedState> {
  RedBloc() : super(const RedState.initial()) {
    on<RedEvent>((event, emit) {
      // TODO: implement event handler
    });
  }
}
