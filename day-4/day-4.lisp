;; To ensure security, a valid passphrase must contain no duplicate words.

;; For example:

;; aa bb cc dd ee is valid.
;; aa bb cc dd aa is not valid - the word aa appears more than once.
;; aa bb cc dd aaa is valid - aa and aaa count as different words.
;; The system's full passphrase list is available as your puzzle input. How many passphrases are valid?

;; --- Part Two ---

;; For added security, yet another system policy has been put in place. Now, a valid passphrase must contain no two words that are anagrams of each other - that is, a passphrase is invalid if any word's letters can be rearranged to form any other word in the passphrase.

;; For example:

;; abcde fghij is a valid passphrase.
;; abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
;; a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
;; iiii oiii ooii oooi oooo is valid.
;; oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.
(ql:quickload :cl-strings)

(defun process(line)
  (when (eql (length line) (length (remove-duplicates line :test #'equal)))
    (return-from process 1)))

(defun ana(line)
  (let ((m (map 'sequence (lambda(x) (sort x #'char>)) line)))
    (when (eql (length line) (length (remove-duplicates m :test #'equal)))
      (return-from ana 1))))

(defun start()
  (let ((first-count 0) (second-count 0) first second)
    (with-open-file (file "./input.txt")
                    (loop for line = (read-line file nil 'end)
                          until (eq line 'end)
                          do (progn
                               (setq first (process (cl-strings:split line " ")))
                               (setq second (ana (cl-strings:split line " ")))
                               (when (numberp first)
                                 (setq first-count (+ first-count first)))
                               (when (numberp second)
                                 (setq second-count (+ second-count second))))
                          finally (print (concatenate 'string "Welcome!
First rule: there are " (write-to-string first-count) " valid passphrases" "
Second rule: there are " (write-to-string second-count) " valid passphrases."))))))
