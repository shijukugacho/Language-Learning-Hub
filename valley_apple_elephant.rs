// 1
pub struct LanguageLearningHub {
	language_schools: Vec<String>,
	language_tutors: Vec<String>,
	language_courses: Vec<String>,
	language_resources: Vec<String>
}

// 2
impl LanguageLearningHub {
	pub fn new() -> Self {
		LanguageLearningHub {
			language_schools: Vec::new(),
			language_tutors: Vec::new(),
			language_courses: Vec::new(),
			language_resources: Vec::new()
		}
	}
	
	pub fn add_language_school(&mut self, school: String) {
		self.language_schools.push(school);
	}
	
	pub fn add_language_tutor(&mut self, tutor: String) {
		self.language_tutors.push(tutor);
	}
	
	pub fn add_language_course(&mut self, course: String) {
		self.language_courses.push(course);
	}
	
	pub fn add_language_resource(&mut self, resource: String) {
		self.language_resources.push(resource);
	}
	
	pub fn get_language_schools(&self) -> &Vec<String> {
		&self.language_schools
	}
	
	pub fn get_language_tutors(&self) -> &Vec<String> {
		&self.language_tutors
	}
	
	pub fn get_language_courses(&self) -> &Vec<String> {
		&self.language_courses
	}
	
	pub fn get_language_resources(&self) -> &Vec<String> {
		&self.language_resources
	}
}

//3
pub struct LanguageSchool {
	name: String,
	location: String,
	teachers: Vec<String>
}

//4
impl LanguageSchool {
	pub fn new(name: String, location: String, teachers: Vec<String>) -> Self {
		LanguageSchool {
			name,
			location,
			teachers
		}
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_location(&self) -> &String {
		&self.location
	}
	
	pub fn get_teachers(&self) -> &Vec<String> {
		&self.teachers
	}
}

//5
pub struct LanguageTutor {
	name: String,
	location: String,
	skill_level: u8
}

//6
impl LanguageTutor {
	pub fn new(name: String, location: String, skill_level: u8) -> Self {
		LanguageTutor {
			name,
			location,
			skill_level
		}
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_location(&self) -> &String {
		&self.location
	}
	
	pub fn get_skill_level(&self) -> u8 {
		self.skill_level
	}
}

//7
pub struct LanguageCourse {
	name: String,
	description: String,
	language: String
}

//8 
impl LanguageCourse {
	pub fn new(name: String, description: String, language: String) -> Self {
		LanguageCourse {
			name,
			description,
			language
		}
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_description(&self) -> &String {
		&self.description
	}
	
	pub fn get_language(&self) -> &String {
		&self.language
	}
}

//9
pub struct LanguageResource {
	name: String,
	type: String,
	language: String
}

//10
impl LanguageResource {
	pub fn new(name: String, type: String, language: String) -> Self {
		LanguageResource {
			name,
			type,
			language
		}
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_type(&self) -> &String {
		&self.type
	}
	
	pub fn get_language(&self) -> &String {
		&self.language
	}
}

//11 
pub fn find_language_schools_in_location(language_learning_hub: &LanguageLearningHub, location: &String) -> Vec<&LanguageSchool> {
	let mut language_schools = Vec::new();
	
	for language_school in &language_learning_hub.language_schools {
		if language_school.location == *location {
			language_schools.push(language_school);
		}
	}
	
	language_schools
}

//12
pub fn find_language_tutors_in_location(language_learning_hub: &LanguageLearningHub, location: &String) -> Vec<&LanguageTutor> {
	let mut language_tutors = Vec::new();
	
	for language_tutor in &language_learning_hub.language_tutors {
		if language_tutor.location == *location {
			language_tutors.push(language_tutor);
		}
	}
	
	language_tutors
}

//13
pub fn find_language_courses_in_language(language_learning_hub: &LanguageLearningHub, language: &String) -> Vec<&LanguageCourse> {
	let mut language_courses = Vec::new();
	
	for language_course in &language_learning_hub.language_courses {
		if language_course.language == *language {
			language_courses.push(language_course);
		}
	}
	
	language_courses
}

//14
pub fn find_language_resources_in_language(language_learning_hub: &LanguageLearningHub, language: &String) -> Vec<&LanguageResource> {
	let mut language_resources = Vec::new();
	
	for language_resource in &language_learning_hub.language_resources {
		if language_resource.language == *language {
			language_resources.push(language_resource);
		}
	}
	
	language_resources
}

//15
pub struct User {
	name: String,
	location: String,
	language: String,
	language_level: u8
}

//16
impl User {
	pub fn new(name: String, location: String, language: String, language_level: u8) -> Self {
		User {
			name,
			location,
			language,
			language_level
		}
	}
	
	pub fn get_name(&self) -> &String {
		&self.name
	}
	
	pub fn get_location(&self) -> &String {
		&self.location
	}
	
	pub fn get_language(&self) -> &String {
		&self.language
	}
	
	pub fn get_language_level(&self) -> u8 {
		self.language_level
	}
}

//17
pub fn find_language_schools_for_user(language_learning_hub: &LanguageLearningHub, user: &User) -> Vec<&LanguageSchool> {
	let mut language_schools = Vec::new();
	
	for language_school in &language_learning_hub.language_schools {
		if language_school.location == user.location && language_school.language == user.language {
			language_schools.push(language_school);
		}
	}
	
	language_schools
}

//18
pub fn find_language_tutors_for_user(language_learning_hub: &LanguageLearningHub, user: &User) -> Vec<&LanguageTutor> {
	let mut language_tutors = Vec::new();
	
	for language_tutor in &language_learning_hub.language_tutors {
		if language_tutor.location == user.location && language_tutor.language == user.language {
			language_tutors.push(language_tutor);
		}
	}
	
	language_tutors
}

//19
pub fn find_language_courses_for_user(language_learning_hub: &LanguageLearningHub, user: &User) -> Vec<&LanguageCourse> {
	let mut language_courses = Vec::new();
	
	for language_course in &language_learning_hub.language_courses {
		if language_course.language == user.language && language_course.language_level <= user.language_level {
			language_courses.push(language_course);
		}
	}
	
	language_courses
}

//20
pub fn find_language_resources_for_user(language_learning_hub: &LanguageLearningHub, user: &User) -> Vec<&LanguageResource> {
	let mut language_resources = Vec::new();
	
	for language_resource in &language_learning_hub.language_resources {
		if language_resource.language == user.language {
			language_resources.push(language_resource);
		}
	}
	
	language_resources
}