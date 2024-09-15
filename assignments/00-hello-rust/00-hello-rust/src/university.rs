#[derive(PartialEq, Clone, Copy, Debug)]
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}

struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    // new students
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lauren",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

fn get_average_gpa_iter<'a>(students: impl Iterator<Item = &'a Student>, len: u32) -> f32 {
    let gpa_sum: f32 = students 
        .map(|student: &Student| student.gpa)
        .sum();
    gpa_sum / (len as f32)
}

fn get_average_gpa() -> f32 {
    let student_list: Vec<&Student> = OLIN_STUDENTS
        .iter()
        .filter(|student| {
            student.class_year != ClassYear::FirstYear
        })
        .collect();
    get_average_gpa_iter(student_list.iter().copied(), student_list.len() as u32)
}

fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let student_list: Vec<&Student> = OLIN_STUDENTS
        .iter()
        .filter(|student| {
            student.class_year == class_year
        })
        .collect();
    let average_gpa: f32 = get_average_gpa();

   student_list 
        .iter()
        .fold(0, |count, student| {
            if student.gpa > average_gpa {
                count+1
            }
            else {
                count
            }
        })
}

fn get_best_class() -> ClassYear {
    *([
        ClassYear::Sophomore,
        ClassYear::Junior,
        ClassYear::Senior,
    ]
        .iter()
        .max_by_key(|class_year: &&ClassYear| {
            get_num_excel_students_for_class(**class_year)
        })
        .unwrap())
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
