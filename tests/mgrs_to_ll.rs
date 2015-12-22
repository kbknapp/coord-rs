extern crate mgrs;

#[test]
fn mgrs_to_ll_point() {
    let ll = LatLon::from_mgrs("33UXP04");

    assert_eq!(ll.lat, 48.2053484);
    assert_eq!(ll.lon, 16.3459270);

    let ll = LatLon::rect_from_mgrs("33UXP04");

    assert_eq!(ll[0].lon, 16.41450); // May need to change to close to
    assert_eq!(ll[0].lat, 0.000001);
    assert_eq!(ll[1].lon, 48.24949); // May need to change to close to
    assert_eq!(ll[1].lat, 0.000001);


    // it('MGRS reference with highest accuracy correct.', function() {
    // mgrs.forward(point).should.equal("33UXP0500444998");
    // it('MGRS reference with 1-digit accuracy correct.', function() {
    // mgrs.forward(point,1).should.equal(mgrsStr);
}

#[test]
fn mgrs_to_ll_point_near_zone_border() {
  // var mgrsStr = "24XWT783908"; // near UTM zone border, so there are two ways to reference this
  // var point = mgrs.toPoint(mgrsStr);
  // it('Longitude of point from MGRS correct.', function() {
  //   point[0].should.be.closeTo(-32.66433, 0.00001);
  // });
  // it('Latitude of point from MGRS correct.', function() {
  //   point[1].should.be.closeTo(83.62778, 0.00001);
  // });
  // it('MGRS reference with 1-digit accuracy correct.', function() {
  //   mgrs.forward(point,3).should.equal('25XEN041865');
  // });
  // it('MGRS reference with 5-digit accuracy, northing all zeros', function(){
  //   mgrs.forward([0,0],5).should.equal('31NAA6602100000');
  // });
  // it('MGRS reference with 5-digit accuracy, northing one digit', function(){
  //   mgrs.forward([0,0.00001],5).should.equal('31NAA6602100001');
  // });
}
