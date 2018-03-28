(defun process(cells)
  (do ((moves 0 (+ 1 moves))
       (position 0))
      ((null (ignore-errors (elt cells position))) (print moves))
    (let* ((spot (elt cells position)) (pos (+ spot position)))
      (setf (elt cells position) (if (>= spot 3)
                                     (- spot 1)
                                     (+ spot 1)))
      (setq position pos))))

(defun start()
  (with-open-file (file "./input.txt")
    (loop for line = (read-line file nil 'end)
       until (eq line 'end)
       collect line into container
       finally (process (map 'vector #'parse-integer container)))))

(time (start))
