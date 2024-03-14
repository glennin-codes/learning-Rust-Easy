
use uuid::Uuid;


#[derive(Debug)]
 #[allow(dead_code)]
enum Subject {
    Mathematics,
    English,
    Physics,
    Geography,
    Chemistry
}
#[derive(Debug)]
struct Grades <T, const N: usize> {
   mathematics: [T; N],
    english: [T; N],
    physics: [T; N],
    geography: [T; N],
    chemistry: [T; N],
}
#[allow(dead_code)]
#[derive(Debug)]
struct Students {
    name: String,
    grades: Grades<i32, 5>,
    regestration_no: Uuid,
    id: u8,
}

impl Students {
    pub fn new_students(name: String, gradings: Grades<i32, 5>, id: u8) -> Vec<Students> {
        let mut students: Vec<Students> = vec![];
        let random_id: Uuid = Uuid::new_v4();

        let student = Self {
            name,
            grades: gradings,
            regestration_no: random_id,
            id,
        };
        students.push(student);
        students
    }
    //update a student's grade for a specific subjects
    #[allow(dead_code)]
    pub fn update_grades(&mut self, subject_name: Subject, grade_index: usize, grade: i32) -> () {
        match subject_name {
            Subject::Mathematics => {
                self.grades.mathematics[grade_index] = grade;
            }
            Subject::English => {
                self.grades.english[grade_index] = grade;
            }
            Subject::Physics => {
                self.grades.physics[grade_index] = grade;
            }
            Subject::Geography => {
                self.grades.geography[grade_index] = grade;
            }
            Subject::Chemistry => {
                self.grades.chemistry[grade_index] = grade;
            }
            
        }
    }
}
//retrieve the average grade for a particular subject across all students

fn calculate_average_subject_grade(
    all_students: &Vec<Students>,
    subject_name: &Subject
) -> std::result::Result<(f32, i32), String>{
    let mut total_marks: i32 = 0;
    let mut count: i32 = 0;
    for students in all_students {
        match subject_name {
          Subject::Mathematics => {
                for grade in students.grades.mathematics {
                    total_marks += grade;
                    count += 1;
                }
            }
           Subject::English=> {
                for grade in students.grades.english {
                    total_marks += grade;
                    count += 1;
                }
            }
            Subject::Physics=> {
                for grade in students.grades.physics {
                    total_marks += grade;
                    count += 1;
                }
            }

            Subject::Chemistry => {
                for grade in students.grades.chemistry {
                    total_marks += grade;
                    count += 1;
                }
            }
            Subject::Geography => {
                for grade in students.grades.geography {
                    total_marks += grade;
                    count += 1;
                }
            }
           
        }
    }
    if count > 0 {
        Ok(((total_marks as f32) / (count as f32),count))
    } else {
        Err(format!("no subject found or no grades for this subject {:?}", subject_name))
    }
}
fn update_user(
    all_students: &mut Vec<Students>,
    id: u8,
    subject_name: Subject,
    grade_index: usize,
    grade: i32
) {
    let getting_student_position: Option<usize> = all_students
        .iter()

        .position(|student: &Students| student.id == id);
    match getting_student_position {
        Some(position) => {
            match all_students.get_mut(position) {
                Some(student) => {
                    student.update_grades(subject_name, grade_index, grade);
                    println!("the updated student is {:?}", student);
                }
                None => println!("failed to get a muitable reference of student"),
            }
        }
        None => println!("No student found with the given id : {}", id),
    }
}

fn main() {
    let mut all_students = Vec::new();
    let grades_student_a: Grades<i32, 5> = Grades {
        mathematics: [70, 65, 90, 78, 80],
        physics: [85, 80, 75, 82, 88],
        chemistry: [29, 33, 38, 45, 57],
        english: [65, 75, 65, 50, 28],
        geography: [65, 45, 88, 60, 54],
    };
    let grades_student_b: Grades<i32, 5> = Grades {
        mathematics: [81, 65, 66, 78, 80],
        physics: [75, 80, 75, 82, 88],
        chemistry: [30, 35, 49, 55, 67],
        english: [67, 70, 55, 50, 39],
        geography: [83, 86, 76, 80, 84],
    };
    let grades_student_c: Grades<i32, 5> = Grades {
        mathematics: [70, 63, 62, 75, 67],
        physics: [85, 84, 74, 71, 90],
        chemistry: [31, 35, 40, 55, 77],
        english: [70, 70, 60, 50, 40],
        geography: [81, 83, 87, 80, 94],
    };
    let grades_student_d: Grades<i32, 5> = Grades {
        mathematics: [91, 87, 88, 75, 88],
        physics: [95, 56, 75, 82, 48],
        chemistry: [34, 5, 94, 55, 97],
        english: [60, 70, 50, 40, 54],
        geography: [89, 83, 88, 90, 94],
    };

    all_students.extend(Students::new_students("Glen Ayienda".to_string(), grades_student_a, 7));
    all_students.extend(Students::new_students("Tucker Carson".to_string(), grades_student_b, 8));
    all_students.extend(Students::new_students("James Williams".to_string(), grades_student_c, 9));
    all_students.extend(Students::new_students("Mike Tyson".to_string(), grades_student_d, 10));

    update_user(&mut all_students, 7, Subject::Chemistry, 1, 90);

    println!("this is the student data of {} students:{:?}", all_students.len(), all_students);
   
   let subject:&Subject=&Subject::Chemistry;

    match calculate_average_subject_grade(&all_students, subject) {
        Ok((average,count)) => println!("the average of {:?} is {:?}   and there are {} students in this class. and total number of exams done by the students for that subject is {}", subject, average, all_students.len(),count),
        Err(message) => println!("{}",message),
    }
}
