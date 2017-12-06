;; The message includes a list of the offsets for each jump. Jumps are relative: -1 moves to the previous instruction, and 2 skips the next one. Start at the first instruction in the list. The goal is to follow the jumps until one leads outside the list.

;; In addition, these instructions are a little strange; after each jump, the offset of that instruction increases by 1. So, if you come across an offset of 3, you would move three instructions forward, but change it to a 4 for the next time it is encountered.

;; How many steps does it take to reach the exit?
(defun process(cells moves position)
  (let ((spot (nth position cells)))
    (if spot
        (progn
          (setq position (+ position (parse-integer spot)))
          (nsubstitute (write-to-string (+ (parse-integer spot) 1)) spot cells)
          (process cells (+ moves 1) position))
        (print moves))))

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
  (let (container)
    (with-open-file (file "./input.txt")
      (loop for line = (read-line file nil 'end)
         until (eq line 'end)
         do (push line container)
         finally (process (reverse container) 0 0)))))
