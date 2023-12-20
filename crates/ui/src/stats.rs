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
                    Cell { "ID" }
                    Cell { "Count" }
                    Cell { "Last" }
                    Cell { "Min" }
                    Cell { "Max" }
                    Cell { "Avg" }
                    Cell { "Freq" }
                    Cell { "Throughput" }
                    Cell { "Jitter" }
                }
                tr {
                    Cell { "" }
                    Cell { "" }
                    Cell { "(ms)" }
                    Cell { "(ms)" }
                    Cell { "(ms)" }
                    Cell { "(ms)" }
                    Cell { "(Hz)" }
                    Cell { "(Hz)" }
                    Cell { "%" }
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

#[derive(Props)]
struct CellProps<'a> {
    children: Element<'a>,
}

fn Cell<'a>(cx: Scope<'a, CellProps<'a>>) -> Element {
    cx.render(rsx!(
        th {
            class: "p-2",
            &cx.props.children
        }
    ))
}

