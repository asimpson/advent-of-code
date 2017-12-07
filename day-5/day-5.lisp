(defun process(cells)
  (do ((moves 0 (+ 1 moves))
       (position 0))
      ((null (ignore-errors (elt cells position))) (print moves))
    (let* ((spot (elt cells position)) (pos (+ spot position)))
      (setf (elt cells position) (+ spot 1))
      (setq position pos))))

(defun check-offset(spot)
  (if (>= spot 3)
      (write-to-string (- spot 1))
      (write-to-string (+ spot 1))))

(defun second-process(cells moves position)
  (let ((spot (nth position cells)))
    (if spot
        (progn
          (setq position (+ position (parse-integer spot)))
          (nsubstitute (check-offset (parse-integer spot)) spot cells)
          (second-process cells (+ moves 1) position))
        (print moves))))

(process '("0" "3" "0" "1" "-3") 0 0)
(second-process '("0" "3" "0" "1" "-3") 0 0)

(defun start()
  (with-open-file (file "./input.txt")
    (loop for line = (read-line file nil 'end)
       until (eq line 'end)
       collect line into container
       finally (process (map 'vector #'parse-integer container)))))

(time (start))
