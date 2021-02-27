(ns s1c1)

(def INPUT "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")

(def EXPECTED "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")

(def encoder (java.util.Base64/getEncoder))

(defn hex-to-base64
  "Converts a hex string to base64"
  [hex]
  (->> hex
       (partition 2)               ; lists of 2 characters               ((4 9) (27) ..)
       (map #(apply str "0x" %))   ; Add 0x at the begining of each pair (0x49 0x27  ..)
       (map #(Long/decode %))      ; Convert each hex to an integer      (73 39      ..)
       (byte-array)
       (.encode encoder)           ; Returns byte array 
       (map char)
       (apply str)))

(defn run []
  (let [result (hex-to-base64 INPUT)]
    (if (= result EXPECTED)
      (println "Sucess at set 1, challenge 1")
      (println
       "Fail at set 1, challenge 1"
       "\nExpected "
       EXPECTED
       "\nGot"
       result))))
