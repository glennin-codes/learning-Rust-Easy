Imagine you are developing a system for managing a school's grading records. Each student has a set of grades for various subjects, and you need to store these grades efficiently. The number of subjects is fixed, but the number of students can vary.

Your task is to design a Rust data structure that can store the grades for a fixed number of subjects for any number of students. The data structure should allow you to easily add new students and their grades, retrieve the average grade for a particular subject across all students, and update a student's grade for a specific subject.

Use const generics to define the number of subjects, and generics to handle the variable number of students. Implement methods for adding students, retrieving averages, and updating grades.