use super::stats_item::StatsItem;
use canbusnoop_db::MultiStats;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub(crate) struct StatsProps {
    stats: MultiStats,
}

pub(crate) fn Stats(cx: Scope<StatsProps>) -> Element {
    let stats = &cx.props.stats;

    cx.render(rsx! {
        table {
            class: "table-fixed w-full text-sm text-left text-gray-500 dark:text-gray-400",
            thead {
                class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                tr {
                    th {
                        class: "w-40",
                        "ID"
                    }
                    th {
                        class: "p-2",
                        "Count"
                    }
                    th {
                        class: "p-2",
                        "Last"
                    }
                    th {
                        class: "p-2",
                        "Min"
                    }
                    th {
                        class: "p-2",
                        "Max"
                    }
                    th {
                        class: "p-2",
                        "Avg"
                    }
                    th {
                        class: "p-2",
                        "Freq"
                    }
                    th {
                        class: "p-2",
                        "Throughput"
                    }
                    th {
                        class: "p-2",
                        "Jitter"
                    }
                }
                tr {
                    th {
                        class: "p-2",
                        ""
                    }
                    th {
                        class: "p-2",
                        ""
                    }
                    th {
                        class: "p-2",
                        "(ms)"
                    }
                    th {
                        class: "p-2",
                        "(ms)"
                    }
                    th {
                        class: "p-2",
                        "(ms)"
                    }
                    th {
                        class: "p-2",
                        "(ms)"
                    }
                    th {
                        class: "p-2",
                        "(Hz)"
                    }
                    th {
                        class: "p-2",
                        "(Hz)"
                    }
                    th {
                        class: "p-2",
                        "%"
                    }
                }
            }
            tbody {
                for (&id, stats) in stats.iter() {
                    StatsItem {
                        id: id,
                        stats: stats.clone()
                    }
                }
            }
        }
    })
}
