use futures::executor;
use grpc::ClientStubExt;
use use_grpc::{
    foobar::{CabLocationRequest, GetCabRequest, Location},
    foobar_grpc::FooBarServiceClient,
};

fn main() {
    let client: FooBarServiceClient =
        FooBarServiceClient::new_plain("127.0.0.1", 9001, Default::default()).unwrap();

    let mut req = CabLocationRequest::new();
    req.set_name("foo".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    let resp = client
        .record_cab_location(grpc::RequestOptions::new(), req)
        .join_metadata_result();
    let resp = executor::block_on(resp);

    match resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, r, _)) => println!("{:?}", r),
    }

    let mut nearby_req = GetCabRequest::new();
    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    nearby_req.set_location(location);

    let nearby_resp = client
        .get_cabs(grpc::RequestOptions::new(), nearby_req)
        .join_metadata_result();

    let nearby_resp = executor::block_on(nearby_resp);

    match nearby_resp {
        Err(e) => panic!("{:?}", e),
        Ok((_, cabs, _)) => println!("{:?}", cabs),
    }
}
