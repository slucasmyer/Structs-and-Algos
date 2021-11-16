package main


func countingSort(arr []int32) []int32 {
    var max int32
    var min int32
    for i := 0; i < len(arr); i++ {
        if i == 0 {
            max = arr[i]
            min = arr[i]
        } else {
            if arr[i] > max {
                max = arr[i]
            }
            if arr[i] < min {
                min = arr[i]
            }
        }
    }
    m := make(map[int32]int32)
    for i := min; i < max+1; i++ {
        m[i] = 0
    }
    for i := int32(0); i < int32(len(arr)); i++ {
        m[arr[i]] += int32(1)
    }
    counts := make([]int32, max+1)
    for i := min; i < max+1; i++ {
        counts[i] = m[i]
    }
    return counts
}