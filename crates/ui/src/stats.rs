use super::stats_item::StatsItem;
use canbusnoop_db::MultiStats;
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub(crate) struct StatsProps {
    stats: MultiStats,
}

pub(crate) fn Stats(cx: Scope<StatsProps>) -> Element {
    let stats = &cx.props.stats;
    let header1 = COLUMNS.iter().map(|(x, _)| render! { Cell { x } });
    let header0 = COLUMNS.iter().map(|(_, x)| render! { Cell { x } });

    render! {
        table {
            class: "table-auto w-full text-sm text-left text-gray-500 dark:text-gray-400",
            thead {
                class: "text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400",
                tr { header0 }
                tr { header1 }
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
    }
}

#[derive(Props)]
struct CellProps<'a> {
    children: Element<'a>,
}

fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element {
    render! {
        th {
            class: "p-2",
            &cx.props.children
        }
    }
}

const COLUMNS: [(&'static str, &'static str); 9] = [
    ("ID", ""),
    ("Count", ""),
    ("Last", "ms"),
    ("Min", "ms"),
    ("Max", "ms"),
    ("Avg", "ms"),
    ("Freq", "Hz"),
    ("Throughput", "Hz"),
    ("Jitter", "%"),
];
