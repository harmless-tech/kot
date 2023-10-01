//// TODO: Redo how benchmarking works.
//
//use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use kot::{args, lexer};
//
//criterion_group!(benches, process_single);
//criterion_main!(benches);
//
//fn process_single(c: &mut Criterion) {
//    c.bench_function("main (single)", |b| {
//        b.iter(|| {
//            let entry_args = args::collect_args();
//
//            let raw_kotfile = std::fs::read_to_string("./test/kotfile2").unwrap();
//            let lexed = lexer::lex(&raw_kotfile);
//
//            black_box(entry_args);
//            black_box(lexed);
//        });
//    });
//}
//
//#[cfg(feature = "threads")]
//fn process_multi(c: &mut Criterion) {
//    c.bench_function("main (multi)", |b| {
//        b.iter(|| {
//            let entry_args = std::thread::spawn(args::collect_args);
//
//            let lexed = std::thread::spawn(|| {
//                // TODO: Temp
//                let raw_kotfile = std::fs::read_to_string("./test/kotfile2").unwrap();
//                lexer::lex(&raw_kotfile)
//            });
//
//            let entry_args = entry_args.join().unwrap();
//            let lexed = lexed.join().unwrap();
//        });
//    });
//}
