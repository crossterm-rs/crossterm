//! # Graphics
//!
//! Terminal capability queries related to inline graphics protocols.

/// Query for [Kitty graphics protocol] support.
///
/// Sends a minimal graphics *query action* in an APC sequence. A terminal that
/// supports the protocol replies with an APC of its own; one that does not
/// stays silent.
///
/// Use with [`QueryBatch`](crate::query::QueryBatch):
///
/// ```no_run
/// # #[cfg(unix)] {
/// use crossterm::graphics::QueryKittyGraphicsSupport;
/// use crossterm::query::QueryBatch;
///
/// let mut batch = QueryBatch::new();
/// let kitty = batch.add(QueryKittyGraphicsSupport);
/// let results = batch.execute()?;
/// println!("kitty graphics: {}", results.get(&kitty)?);
/// # }
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// [Kitty graphics protocol]: https://sw.kovidgoyal.net/kitty/graphics-protocol/#querying-support-and-available-transmission-mediums
#[cfg(all(unix, feature = "events"))]
#[derive(Clone)]
pub struct QueryKittyGraphicsSupport;

#[cfg(all(unix, feature = "events"))]
#[allow(private_interfaces)]
impl crate::query::TerminalQuery for QueryKittyGraphicsSupport {
    type Response = bool;

    fn query_bytes(&self) -> Vec<u8> {
        b"\x1B_Gi=31,s=1,v=1,a=q,t=d,f=24;AAAA\x1B\\".to_vec()
    }

    fn matches(&self, event: &crate::event::internal::InternalEvent) -> bool {
        matches!(
            event,
            crate::event::internal::InternalEvent::KittyGraphicsSupportResponse
        )
    }

    fn extract(
        &self,
        event: Option<crate::event::internal::InternalEvent>,
    ) -> std::io::Result<bool> {
        Ok(matches!(
            event,
            Some(crate::event::internal::InternalEvent::KittyGraphicsSupportResponse)
        ))
    }
}
