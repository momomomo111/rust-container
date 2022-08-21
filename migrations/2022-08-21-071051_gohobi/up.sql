CREATE TABLE gohobi (
    id INT PRIMARY KEY AUTO_INCREMENT,
    message VARCHAR (64) NOT NULL,
    from_user_id INT NOT NULL,
    task_id INT NOT NULL,
    FOREIGN KEY (from_user_id) REFERENCES user (id),
    FOREIGN KEY (task_id) REFERENCES task (id)
);