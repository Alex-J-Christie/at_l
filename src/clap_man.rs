/*
okay, let's think about what we want
add a main activity should look like this
at_l --add_activity Writing, maybe we can shorten that to -a?
add a sub activity, which should take two strings
at_l --add_sub_activity Writing BermudaW, shorten to -s
add an entry for a specific activity
at_l --count_activity_time BermudaW, shorten to -c
print out the activities
at_l --get_activities, shorten to -g
print out the sub activities
at_l --get_sub_activities Writing, shorten to -b
print out ALL activities,
at_l --get_all_activities, shorten to -G
print out activity time from main table
at_l --get_time Writing, shorten to -t
print out a specific number of recent entries
at_l --get_sub_time Writing 5, shorten to -T, (this gets the 5 most recent entries)
start gui when I get that done
at_l --gui, shorten to -E
 */

    use clap::{Arg, arg, ArgMatches, command};
    use iced::{Application, Settings};
    use crate::activity_data::ActivityData;
    use crate::time_results::TimeResults;
    use crate::db_manager::db_man as db;
    use crate::gui_man;
    use text_io::read;
    use crate::db_manager::db_man::{get_times_by_date, return_main_tables, return_sub_tables};

pub fn run_args(args: ArgMatches) {
        if let Some(a) = args.get_one::<String>("add_activity") {
            db::add_primary_activity(a);
        }
        if let Some(s) = args.get_one::<String>("add_sub_activity") {
            println!("Please type in the name of your sub-activity");
            let test = read!();
            db::add_sub_activity(s, test);
        }
        if let Some(c) = args.get_one::<String>("count_activity_time") {
            let time = TimeResults::new().count_time();
            let activity = ActivityData::new(c, time);
            db::add_entry(activity);
        }
        if let Some(g) = args.get_one::<bool>("get_activities") {
            if *g {
                let flushed: Vec<String> = return_main_tables();
                for x in flushed.into_iter() {
                    println!("{}", x);
                }
            }
        }
        if let Some(b) = args.get_one::<String>("get_sub_activities") {
            let flushed: Vec<String> = return_sub_tables(b);
            for (x, y) in flushed.into_iter().enumerate() {
                println!("{}: {} - {} seconds", (x + 1), y, db::return_table_time_total(&y));
            }
        }
        if let Some(g) = args.get_one::<bool>("get_all_activities") {
            if *g {
                let flushed: Vec<String> = db::return_tables();
                for (x, y) in flushed.into_iter().enumerate() {
                    println!("{}: {} - {} seconds", (x + 1), y, db::return_table_time_total(&y));
                }
            };
        }
        if let Some(t) = args.get_one::<String>("get_time") {
            let print_time: u64 = db::return_table_time_total(t);
            println!("Hours: {}, Minutes: {}, Seconds: {}, where the total time (in seconds) is: {}", print_time / 60 / 60, print_time / 60, print_time & 60, print_time);
        }
        if let Some(t) = args.get_one::<String>("get_time_entries") {
            println!("How many entries do you want to read?");
            let entries_num: u64 = read!();
            db::print_table_rows(t, entries_num);
        }
        if let Some(o) = args.get_one::<String>("get_sub_times") {
            let act_names: Vec<String> = return_sub_tables(o);
            let print_times: Vec<u64> = db::get_sub_times(o);
            println!("{:?}", act_names);
            println!("{:?}", print_times);
        }
        if let Some(t) = args.get_one::<bool>("get_all_info") {
            if *t {
                println!("{:#?}", db::get_all_sub_times());
            }
        }
        if let Some(m) = args.get_one::<String>("get_month_info") {
            let name = m;
            for e in return_sub_tables(name).iter() {
                for day in 13..30 {
                    let day_check = get_times_by_date(e.to_string(), day);
                    if day_check.3 != 0 {
                        println!("{:?}", day_check);
                    }
                }
            }
        }
    // this is shit, what is this? fix this at some point jesus christ
    if let Some(t) = args.get_one::<bool>("get_all_month_info") {
        if *t {
            let tables = return_main_tables();
            for main_activity in tables {
                for sub_activity in return_sub_tables(&main_activity) {
                    for day in 13..30 {
                        let day_check = get_times_by_date(sub_activity.to_string(), day);
                        if day_check.3 != 0 {
                            println!("{}", day_check.0);
                            println!("{}", day_check.1);
                            println!("{}", day_check.2);
                            println!("{}", day_check.3);
                        }
                    }
                }
            }
        }
    }
        if let Some(e) = args.get_one::<bool>("gui") {
            if *e {
                gui_man::Window::run(Settings::default()).expect("TODO: panic message");
            };
        }
    }
    pub fn return_arg_array() -> ArgMatches {
        let add_activity: Arg = arg!(-a --add_activity <Main_Activity> "Adds a primary activity (Something basic like chores, or studying");
        let add_sub_activity: Arg = arg!(-s --add_sub_activity <Main_Activity> "Adds a sub activity (E.G. Add Statistics to Studying)");
        let count_activity_time: Arg = arg!(-c --count_activity_time <Activity> "Starts a timer for a specified activity");
        let get_activities: Arg = arg!(-g --get_activities "Lists all main activities");
        let get_sub_activities: Arg = arg!(-b --get_sub_activities <Main_Activity> "Lists all sub activities for an activity");
        let get_all_activities: Arg = arg!(-G --get_all_activities "Lists all activities and sub activities");
        let get_time: Arg = arg!(-t --get_time <Activity> "Lists the total time for an activity and all sub activities");
        let get_time_entries: Arg = arg!(-T --get_time_entries <Activity> "Lists the 5 most recent time entries for an activity");
        let get_sub_times: Arg = arg!(-o --get_sub_times <Activity> "Lists total times for Sub Activities of a Main Activity");
        let get_all_info: Arg = arg!(-y --get_all_info "Lists all of it");
        let get_month_info: Arg = arg!(-m --get_month_info <Activity> "Lists info for the month of november");
        let get_all_month_info: Arg = arg!(-M --get_all_month_info "Lists all info for the month of november");
        let gui: Arg = arg!(-E --gui "Runs the gui version of at_l");

        command!().args([
            add_activity,
            add_sub_activity,
            count_activity_time,
            get_activities,
            get_sub_activities,
            get_all_activities,
            get_time,
            get_time_entries,
            get_sub_times,
            get_all_info,
            get_month_info,
            get_all_month_info,
            gui
        ]).get_matches()
    }


