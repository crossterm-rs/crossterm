//! # Graphics
//!
//! Terminal capability queries related to inline graphics protocols.

/// Query for Sixel graphics support.
///
/// Sixel support is reported as attribute `4` in the [DA1] response. `QueryBatch`
/// already sends DA1 as its sentinel, so this query writes no bytes of its own
/// and simply inspects the existing DA1 reply.
///
/// Use with [`QueryBatch`](crate::query::QueryBatch):
///
/// ```no_run
/// # #[cfg(unix)] {
/// use crossterm::graphics::QuerySixelSupport;
/// use crossterm::query::QueryBatch;
///
/// let mut batch = QueryBatch::new();
/// let sixel = batch.add(QuerySixelSupport);
/// let results = batch.execute()?;
/// println!("sixel: {}", results.get(&sixel)?);
/// # }
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// [DA1]: https://vt100.net/docs/vt510-rm/DA1.html
#[cfg(all(unix, feature = "events"))]
#[derive(Clone)]
pub struct QuerySixelSupport;

#[cfg(all(unix, feature = "events"))]
#[allow(private_interfaces)]
impl crate::query::TerminalQuery for QuerySixelSupport {
    type Response = bool;

    fn query_bytes(&self) -> Vec<u8> {
        Vec::new()
    }

    fn matches(&self, event: &crate::event::internal::InternalEvent) -> bool {
        matches!(
            event,
            crate::event::internal::InternalEvent::PrimaryDeviceAttributes(_)
        )
    }

    fn extract(
        &self,
        event: Option<crate::event::internal::InternalEvent>,
    ) -> std::io::Result<bool> {
        match event {
            Some(crate::event::internal::InternalEvent::PrimaryDeviceAttributes(attrs)) => {
                Ok(attrs.contains(&4))
            }
            None => Ok(false),
            _ => unreachable!(),
        }
    }
}
