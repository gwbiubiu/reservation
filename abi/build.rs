fn main(){
    tonic_build::configure()
        .out_dir("src/pb").
        compile(&["protos/reservation.proto"], &["proto"]).unwrap();
}