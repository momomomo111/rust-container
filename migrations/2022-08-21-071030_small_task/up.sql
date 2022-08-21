CREATE TABLE small_task (
    id INT PRIMARY KEY AUTO_INCREMENT,
    small_task_name VARCHAR (64),
    task_id INT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES task (id)
);