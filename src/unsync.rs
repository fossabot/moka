//! Provides a *not* thread-safe, blocking cache implementation.

pub(crate) mod builder;
pub(crate) mod cache;
mod deques;

use std::{ptr::NonNull, rc::Rc};

pub use builder::CacheBuilder;
pub use cache::Cache;
use quanta::Instant;

use crate::common::{deque::DeqNode, AccessTime};

pub(crate) struct KeyDate<K> {
    pub(crate) key: Rc<K>,
    pub(crate) timestamp: Option<Instant>,
}

impl<K> KeyDate<K> {
    pub(crate) fn new(key: Rc<K>, timestamp: Option<Instant>) -> Self {
        Self { key, timestamp }
    }
}

pub(crate) struct KeyHashDate<K> {
    pub(crate) key: Rc<K>,
    pub(crate) hash: u64,
    pub(crate) timestamp: Option<Instant>,
}

impl<K> KeyHashDate<K> {
    pub(crate) fn new(key: Rc<K>, hash: u64, timestamp: Option<Instant>) -> Self {
        Self {
            key,
            hash,
            timestamp,
        }
    }
}

// DeqNode for an access order queue.
type KeyDeqNodeAo<K> = NonNull<DeqNode<KeyHashDate<K>>>;

// DeqNode for the write order queue.
type KeyDeqNodeWo<K> = NonNull<DeqNode<KeyDate<K>>>;

struct DeqNodes<K> {
    access_order_q_node: Option<KeyDeqNodeAo<K>>,
    write_order_q_node: Option<KeyDeqNodeWo<K>>,
}

pub(crate) struct ValueEntry<K, V> {
    pub(crate) value: V,
    deq_nodes: DeqNodes<K>,
}

impl<K, V> ValueEntry<K, V> {
    pub(crate) fn new(value: V) -> Self {
        Self {
            value,
            deq_nodes: DeqNodes {
                access_order_q_node: None,
                write_order_q_node: None,
            },
        }
    }

    #[inline]
    pub(crate) fn replace_deq_nodes_with(&mut self, mut other: Self) {
        self.deq_nodes.access_order_q_node = other.deq_nodes.access_order_q_node.take();
        self.deq_nodes.write_order_q_node = other.deq_nodes.write_order_q_node.take();
    }

    #[inline]
    pub(crate) fn access_order_q_node(&self) -> Option<KeyDeqNodeAo<K>> {
        self.deq_nodes.access_order_q_node
    }

    #[inline]
    pub(crate) fn set_access_order_q_node(&mut self, node: Option<KeyDeqNodeAo<K>>) {
        self.deq_nodes.access_order_q_node = node;
    }

    #[inline]
    pub(crate) fn take_access_order_q_node(&mut self) -> Option<KeyDeqNodeAo<K>> {
        self.deq_nodes.access_order_q_node.take()
    }

    #[inline]
    pub(crate) fn write_order_q_node(&self) -> Option<KeyDeqNodeWo<K>> {
        self.deq_nodes.write_order_q_node
    }

    #[inline]
    pub(crate) fn set_write_order_q_node(&mut self, node: Option<KeyDeqNodeWo<K>>) {
        self.deq_nodes.write_order_q_node = node;
    }

    #[inline]
    pub(crate) fn take_write_order_q_node(&mut self) -> Option<KeyDeqNodeWo<K>> {
        self.deq_nodes.write_order_q_node.take()
    }
}

impl<K, V> AccessTime for ValueEntry<K, V> {
    #[inline]
    fn last_accessed(&self) -> Option<Instant> {
        self.access_order_q_node()
            .and_then(|node| unsafe { node.as_ref() }.element.timestamp)
    }

    #[inline]
    fn set_last_accessed(&mut self, timestamp: Instant) {
        if let Some(mut node) = self.deq_nodes.access_order_q_node {
            unsafe { node.as_mut() }.set_last_accessed(timestamp);
        }
    }

    #[inline]
    fn last_modified(&self) -> Option<Instant> {
        self.write_order_q_node()
            .and_then(|node| unsafe { node.as_ref() }.element.timestamp)
    }

    #[inline]
    fn set_last_modified(&mut self, timestamp: Instant) {
        if let Some(mut node) = self.deq_nodes.write_order_q_node {
            unsafe { node.as_mut() }.set_last_modified(timestamp);
        }
    }
}

impl<K> AccessTime for DeqNode<KeyDate<K>> {
    #[inline]
    fn last_accessed(&self) -> Option<Instant> {
        None
    }

    #[inline]
    fn set_last_accessed(&mut self, _timestamp: Instant) {
        unreachable!();
    }

    #[inline]
    fn last_modified(&self) -> Option<Instant> {
        self.element.timestamp
    }

    #[inline]
    fn set_last_modified(&mut self, timestamp: Instant) {
        self.element.timestamp = Some(timestamp);
    }
}

impl<K> AccessTime for DeqNode<KeyHashDate<K>> {
    #[inline]
    fn last_accessed(&self) -> Option<Instant> {
        self.element.timestamp
    }

    #[inline]
    fn set_last_accessed(&mut self, timestamp: Instant) {
        self.element.timestamp = Some(timestamp);
    }

    #[inline]
    fn last_modified(&self) -> Option<Instant> {
        None
    }

    #[inline]
    fn set_last_modified(&mut self, _timestamp: Instant) {
        unreachable!();
    }
}
