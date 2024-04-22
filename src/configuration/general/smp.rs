use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Configuration for a SMP system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmpConfig {
    // Simulate a SMP system with 'n' CPUs initially present on the  machine  type  board.
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<usize>,

    //  On boards  supporting  CPU  hotplug, the optional 'maxcpus' parameter can be set to
    // enable further CPUs to be added at runtime.
    // When both parameters are omitted, the maximum num‐
    // ber  of  CPUs will be calculated from the provided topology members and the initial CPU
    // count will match the maximum number. When only one of them is given  then  the  omitted
    // one  will be set to its counterpart's value.  Both parameters may be specified, but the
    // maximum number of CPUs must be equal to or greater than the initial CPU count. Both pa‐
    // rameters  are subject to an upper limit that is determined by the specific machine type
    // chosen.
    #[serde(rename = "maxcpus", skip_serializing_if = "Option::is_none")]
    pub maxcpus: Option<usize>,

    // drawers= number of drawers on the machine board.
    #[serde(rename = "drawers", skip_serializing_if = "Option::is_none")]
    pub drawers: Option<usize>,

    // books= number of books in one drawer.
    #[serde(rename = "books", skip_serializing_if = "Option::is_none")]
    pub books: Option<usize>,

    // sockets= number of sockets in one book.
    #[serde(rename = "sockets", skip_serializing_if = "Option::is_none")]
    pub sockets: Option<usize>,

    // dies= number of dies in one socket.
    #[serde(rename = "dies", skip_serializing_if = "Option::is_none")]
    pub dies: Option<usize>,

    // clusters= number of clusters in one die.
    #[serde(rename = "clusters", skip_serializing_if = "Option::is_none")]
    pub clusters: Option<usize>,

    // cores= number of cores in one cluster.
    #[serde(rename = "cores", skip_serializing_if = "Option::is_none")]
    pub cores: Option<usize>,

    // threads= number of threads in one core.
    #[serde(rename = "threads", skip_serializing_if = "Option::is_none")]
    pub threads: Option<usize>,
}

impl<'a> SmpConfig {
    #[inline]
    fn cpus(&self) -> Option<KVArgQ<'a>> {
        match self.cpus {
            None => None,
            Some(cpus) => Some(KVArgQ {
                key: "cpus",
                kv_split_with: Some("="),
                value: Some(cpus.to_string()),
            }),
        }
    }

    #[inline]
    fn maxcpus(&self) -> Option<KVArgQ<'a>> {
        match self.maxcpus {
            None => None,
            Some(maxcpus) => Some(KVArgQ {
                key: "maxcpus",
                kv_split_with: Some("="),
                value: Some(maxcpus.to_string()),
            }),
        }
    }

    #[inline]
    fn drawers(&self) -> Option<KVArgQ<'a>> {
        match self.drawers {
            None => None,
            Some(drawers) => Some(KVArgQ {
                key: "drawers",
                kv_split_with: Some("="),
                value: Some(drawers.to_string()),
            }),
        }
    }

    #[inline]
    fn books(&self) -> Option<KVArgQ<'a>> {
        match self.books {
            None => None,
            Some(books) => Some(KVArgQ {
                key: "books",
                kv_split_with: Some("="),
                value: Some(books.to_string()),
            }),
        }
    }

    #[inline]
    fn sockets(&self) -> Option<KVArgQ<'a>> {
        match self.sockets {
            None => None,
            Some(sockets) => Some(KVArgQ {
                key: "sockets",
                kv_split_with: Some("="),
                value: Some(sockets.to_string()),
            }),
        }
    }

    #[inline]
    fn dies(&self) -> Option<KVArgQ<'a>> {
        match self.sockets {
            None => None,
            Some(dies) => Some(KVArgQ {
                key: "dies",
                kv_split_with: Some("="),
                value: Some(dies.to_string()),
            }),
        }
    }

    #[inline]
    fn clusters(&self) -> Option<KVArgQ<'a>> {
        match self.clusters {
            None => None,
            Some(clusters) => Some(KVArgQ {
                key: "clusters",
                kv_split_with: Some("="),
                value: Some(clusters.to_string()),
            }),
        }
    }

    #[inline]
    fn cores(&self) -> Option<KVArgQ<'a>> {
        match self.cores {
            None => None,
            Some(cores) => Some(KVArgQ {
                key: "cores",
                kv_split_with: Some("="),
                value: Some(cores.to_string()),
            }),
        }
    }

    #[inline]
    fn threads(&self) -> Option<KVArgQ<'a>> {
        match self.threads {
            None => None,
            Some(threads) => Some(KVArgQ {
                key: "threads",
                kv_split_with: Some("="),
                value: Some(threads.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for SmpConfig {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "smp",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![
                self.cpus(),
                self.maxcpus(),
                self.drawers(),
                self.books(),
                self.sockets(),
                self.dies(),
                self.clusters(),
                self.cores(),
                self.threads(),
            ]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect(),
        }
    }
}
