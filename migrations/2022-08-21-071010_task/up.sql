CREATE TABLE task (
    id INT PRIMARY KEY AUTO_INCREMENT,
    task_name VARCHAR (64) NOT NULL,
    dead_line TIMESTAMP NOT NULL,
    user_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id)
);