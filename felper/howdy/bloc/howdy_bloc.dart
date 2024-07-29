import 'package:bloc/bloc.dart';
import 'package:freezed_annotation/freezed_annotation.dart';

part 'howdy_event.dart';
part 'howdy_state.dart';
part 'howdy_bloc.freezed.dart';

class HowdyBloc extends Bloc<HowdyEvent, HowdyState> {
  HowdyBloc() : super(const HowdyState.initial()) {
    on<HowdyEvent>((event, emit) {
      // TODO: implement event handler
    });
  }
}
