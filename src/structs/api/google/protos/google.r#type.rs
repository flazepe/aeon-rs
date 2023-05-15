/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="<http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84>
/// standard</a>. Values must be within normalized ranges.
///
/// Example of normalization code in Python:
///
///      def NormalizeLongitude(longitude):
///        """Wraps decimal degrees longitude to [-180.0, 180.0]."""
///        q, r = divmod(longitude, 360.0)
///        if r > 180.0 or (r == 180.0 and q <= -1.0):
///          return r - 360.0
///        return r
///
///      def NormalizeLatLng(latitude, longitude):
///        """Wraps decimal degrees latitude and longitude to
///        [-90.0, 90.0] and [-180.0, 180.0], respectively."""
///        r = latitude % 360.0
///        if r <= 90.0:
///          return r, NormalizeLongitude(longitude)
///        elif r >= 270.0:
///          return r - 360, NormalizeLongitude(longitude)
///        else:
///          return 180 - r, NormalizeLongitude(longitude + 180.0)
///
///      assert 180.0 == NormalizeLongitude(180.0)
///      assert -180.0 == NormalizeLongitude(-180.0)
///      assert -179.0 == NormalizeLongitude(181.0)
///      assert (0.0, 0.0) == NormalizeLatLng(360.0, 0.0)
///      assert (0.0, 0.0) == NormalizeLatLng(-360.0, 0.0)
///      assert (85.0, 180.0) == NormalizeLatLng(95.0, 0.0)
///      assert (-85.0, -170.0) == NormalizeLatLng(-95.0, 10.0)
///      assert (90.0, 10.0) == NormalizeLatLng(90.0, 10.0)
///      assert (-90.0, -10.0) == NormalizeLatLng(-90.0, -10.0)
///      assert (0.0, -170.0) == NormalizeLatLng(-180.0, 10.0)
///      assert (0.0, -170.0) == NormalizeLatLng(180.0, 10.0)
///      assert (-90.0, 10.0) == NormalizeLatLng(270.0, 10.0)
///      assert (90.0, 10.0) == NormalizeLatLng(-270.0, 10.0)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
