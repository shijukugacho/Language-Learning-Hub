import UIKit

class LanguageLearningHub {
    // Properties
    private var userName: String
    private var password: String
    private var languageList: [String]
    private var learningLanguage: String
    private var currentLevel: Int
    
    // Initializers
    init(username: String, password: String, languageList: [String], learningLanguage: String, currentLevel: Int) {
        self.userName = username
        self.password = password
        self.languageList = languageList
        self.learningLanguage = learningLanguage
        self.currentLevel = currentLevel
    }
    
    // Methods
    func logIn() {
        // code to log in
    }
    
    func selectLanguage() {
        // code to select language
    }
    
    func viewLanguageList() {
        // code to view language list
    }
    
    func viewCurrentLevel() {
        // code to view current level
    }
    
    func startLesson() {
        // code to start lesson
    }
    
    func quiz() {
        // code to quiz
    }
    
    func takeTest() {
        // code to take test
    }
    
    func updateProgress() {
        // code to update progress
    }
    
    func logOut() {
        // code to log out
    }
    
    func getUsername() -> String {
        return self.userName
    }
    
    func setUsername(username: String) {
        self.userName = username
    }
    
    func getPassword() -> String {
        return self.password
    }
    
    func setPassword(password: String) {
        self.password = password
    }
    
    func getLanguageList() -> [String] {
        return self.languageList
    }
    
    func setLanguageList(languageList: [String]) {
        self.languageList = languageList
    }
    
    func getLearningLanguage() -> String {
        return self.learningLanguage
    }
    
    func setLearningLanguage(learningLanguage: String) {
        self.learningLanguage = learningLanguage
    }
    
    func getCurrentLevel() -> Int {
        return self.currentLevel
    }
    
    func setCurrentLevel(currentLevel: Int) {
        self.currentLevel = currentLevel
    }
    
    static func register(username: String, password: String) {
        // code to register
    }
    
    static func forgotPassword(username: String) {
        // code to reset password
    }
    
    static func resumeLearning(username: String, password: String) {
        // code to resume learning
    }
    
}