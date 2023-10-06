use std::collections::HashMap;

fn main(){
    let arr: Vec<i32>= vec![1,2,3,4,5,6,7,8,10];
    let arr_length:i32= arr.len() as i32;
    let final_index:i32 = binary_search(&arr,7,0,arr_length);
    println!("{}",final_index);
    //----------------------------------------------------

    let char_arr:Vec<&str> = vec!["a","b","c","d","e","f","g","h","i","j"];
    let char_index: i32 = binary_search_for_letters(&char_arr, "b", 0, char_arr.len() as i32);
    println!("{}", char_index);

    //-------------------------------------------------------
    println!("find number of rotation");
    let num_arr: Vec<i32> = vec![80, 90, 100,110, 10, 20, 30, 40, 50, 60];
    let start:i32 = 0;
    let end:i32 = num_arr.len() as i32;
    let num_of_rotation = find_number_of_rotation(&num_arr,  start, end-1);
    println!("number of rotation {:?}", num_of_rotation);


    //------------------------------------------------------------------
    println!("find index of target in circular sorted array");
    let num_arr: Vec<i32> = vec![12,13,14,15,1,2,3,4,5,6,7,8,9,10,11];
    let result :i32 = search_circular_sorted_array(&num_arr, 15, 0, num_arr.len()-1);
    println!("index in cicular array, {}", result); 


    //---------------------------------------correct exit conditions
    println!("find first occurance");
    //[1,2,2,3,3,3,4,4,5,5,6,6,6,6]
    let num_arr: Vec<i32> = vec![1,2,2,3,3,3,4,4,5,5,6,6,6,6,70];
    let result :i32 = finding_first_occurance(&num_arr, 5, 0, (num_arr.len()-1) as i32,-1);
    println!("first occurance, {}", result); 


    //---------------------------------------correct exit conditions
    println!("find last occurance");
    //[1,2,2,3,3,3,4,4,5,5,6,6,6,6]
    let num_arr: Vec<i32> = vec![1,2,2,3,3,3,4,4,5,5,6,6,6,6,70];
    let result :i32 = finding_last_occurance(&num_arr, 6, 0, (num_arr.len()-1) as i32,-1);
    println!("last occurance, {}", result); 


    //------------------------------------correct exit conditions
    println!("find missing elements");
    let arr: Vec<i32>= vec![1,2,3,4,5,6,7,8,10];
    let missing_elem = finding_missing_element(&arr, 0, (arr.len() -1) as i32,-1);
    println!("missing elem, {}", missing_elem);


    //------------------------------------correct exit conditions
    println!("find floor");
    let arr: Vec<i32>= vec![1,2,7,8,10];
    for i in 0..12{
        let floor = find_floor(&arr, i,0, (arr.len() -1) as i32,-1);
        let ciel = find_ciel(&arr, i,0, (arr.len() -1) as i32,-1);
        println!("number {}, floor, {}, ciel {}",i, floor, ciel);
    }

    //------------------------------------sqrt
    println!("find sqrt");
    let num = 120;
    let sqrt = sqrt(num,num/2,-1);
    println!("sqrt {}",sqrt);
    
    //------------------------------------correct  conditions
    println!("find frequency of numbers");
    let arr: Vec<i32>= vec![2, 2, 2, 4, 4, 4, 5, 5, 6, 8, 8, 9];
    let mut freq:HashMap<i32, i32> = HashMap::new();
    let freq_array = find_frequency_of_each_element(&arr, 0, (arr.len() -1) as i32, freq);
    println!("frequency , {:?}", freq_array);



    //------------------------------------correct  conditions
     println!("find occurance");
     let arr: Vec<i32>= vec![2, 2, 1, 1, 3, 3, 2, 2, 4, 4, 1, 1,3,3];
     let odd_occurance = find_odd_occurance(&arr, 0, (arr.len() -1) as i32);
     println!("odd occurance , {:?}", odd_occurance);

    //--------------------------------------------
    println!("find pairs");
    let arr: Vec<i32>= vec![1,2,3,4,5,6,7,8,10];
    find_pair(&arr,2);


    //--------------------------------------------
    println!("find pairs");
    let arr: Vec<i32>= vec![1,2,3,4,5,6,7,8,10];
    let closestes = find_closest(&arr,3,2,0,(arr.len() -1 )as i32);
    println!("closests {:?}", closestes);

}

fn binary_search(arr: &Vec<i32>, target:i32,start: i32, end:i32)-> i32{
    println!("{:?}",arr);
    // println!("start {}, end {}", start, end);
    let avg:i32 = (start + end) / 2;
    // println!(" avg {}",avg);
    if start >= end{
        println!("not found");
        return -1
    }
    if target > avg{
        binary_search(&arr, target,avg+1, end)

    }
    else if target < avg {
        binary_search(&arr, target,start, avg-1)   
    }
    else{
        let target_index =  arr.iter().position(|&value|{ 
            if value==target{
                true
            }else{
                false
            }
        }).unwrap();
        target_index as i32
    }
}

fn binary_search_for_letters(arr: &Vec<&str>, target:&str, start: i32, end:i32)->i32{
    // println!("starting string");
    // println!("{:?}",arr);
    // println!("start {}, end {}", start, end);
    let avg:i32 = (start + end) / 2;
    // println!(" avg {} {}, {}",avg,arr[avg as usize], target);

    if start > end{
        println!("not found");
        return -1
    }

    if  arr[avg as usize] < target{
        return binary_search_for_letters(&arr, target, avg+1, end)
    }else if arr[avg as usize] > target{
        return binary_search_for_letters(&arr, target, start, avg-1)
    }else{
        //if wanted to compare index with target    `
        //arr[avg as usize] == target
        let target_index:i32 = arr.iter().position(|&value|{
            if value == target{
                true
            }else{
                false
            }
        }).unwrap() as i32;
        return target_index
    }
}

fn find_number_of_rotation(num_arr:&Vec<i32>, start:i32, end:i32)-> i32{
    //[80, 90, 100,110, 10, 20, 30, 40, 50,60]
    let mid = (start + end) / 2 ;
    let next = (mid + 1) as usize % num_arr.len();
    let prev = ((mid -1) as usize + num_arr.len()) % num_arr.len();
    // let end = (end -1) as usize;
    // println!("mid {}, start {}, end {}, next {}, prev {}",mid, start, end, next, prev);
    if num_arr[mid as usize] < num_arr[start as usize] {
         find_number_of_rotation(&num_arr, start+1, mid)
    }else if num_arr[mid as usize] > num_arr[end as usize] {
        find_number_of_rotation(&num_arr, mid + 1, end)
    }else{
        if num_arr[mid as usize] < num_arr[next] && num_arr[mid as usize] < num_arr[prev] {
            return mid;
        }else{
            -1
        }
    }

}

fn search_circular_sorted_array(num_arr:&Vec<i32>, target:i32, start: usize, end:usize)->i32{
    //[12,13,14,15,1,2,3,4,5,6,7,8,9,10,11]
    //start=0, end=14, mid=7, tgt=15
    let mid = (start + end) / 2;
    if num_arr[mid] == target{
        return mid as i32;
    }
    if start >= end {
        return -1

    }
    if num_arr[mid]<=num_arr[end]{
        if num_arr[mid] < target && target <= num_arr[end]{
            return search_circular_sorted_array(num_arr, target, mid+1, end)
        }else{
           return search_circular_sorted_array(num_arr, target, start, mid -1)
        }
    }else{
        if num_arr[mid] > target && target >= num_arr[start]{
            return search_circular_sorted_array(num_arr, target, start, mid -1)
        }else{
            return search_circular_sorted_array(num_arr, target, mid + 1, end)
        }
    }
}


fn finding_first_occurance(num_arr:&Vec<i32>, target:i32, start: i32, end:i32, result:i32)->i32{
    //[1,2,2,3,3,3,4,4,5,5,6,6,6,6]
    //start=0, end=14, mid=7, tgt=15
    let mid = (start + end) / 2;
    // println!("start {}, end {}, mid {}", start, end, mid);
    if start > end {
        // let index = num_arr.iter().position(|&value|value==target).unwrap();
        println!("from start,end scope");
        return result
    }
    if num_arr[mid as usize]==target{
        // let index = num_arr.iter().position(|&value|value==target).unwrap();
        return finding_first_occurance(&num_arr, target, start, mid -1, mid);
    }
    if num_arr[mid as usize] > target {
        return finding_first_occurance(num_arr, target, start, mid -1,result);
    }else{
        return finding_first_occurance(num_arr, target, mid + 1, end, result);
    }
}


fn finding_last_occurance(num_arr:&Vec<i32>, target:i32, start: i32, end:i32, result:i32)->i32{
    //[1,2,2,3,3,3,4,4,5,5,6,6,6,6]
    //start=0, end=14, mid=7, tgt=15
    let mid = (start + end) / 2;
    // println!("start {}, end {}, mid {}", start, end, mid);
    if start > end {
        // let index = num_arr.iter().position(|&value|value==target).unwrap();
        println!("from start,end scope");
        return result
    }
    if num_arr[mid as usize]==target{
        // let index = num_arr.iter().position(|&value|value==target).unwrap();
        return finding_last_occurance(&num_arr, target, mid + 1, end, mid);
    }
    if num_arr[mid as usize] > target {
        return finding_last_occurance(num_arr, target, start, mid -1,result);
    }else{
        return finding_last_occurance(num_arr, target, mid + 1, end, result);
    }
}

fn finding_missing_element(num_arr:&Vec<i32>, start: i32, end:i32, result:i32)->i32{
    let mid = (start + end)/2;
    // println!("mid {}, start {}, end {}",mid, start, end);
    if start > end {
        return start
    }
    if mid == num_arr[mid as usize] {
        return finding_missing_element(&num_arr, mid + 1, end, result);
    }else{
        return finding_missing_element(num_arr, start, mid -1,mid);
    }
}

fn find_floor(num_arr:&Vec<i32>, target:i32, start: i32, end:i32, result:i32) -> i32{
    let mid = (start + end )/ 2;
    if start > end{
        return result;
    }
    if num_arr[mid as usize] == target {
        return target;
    }
    if num_arr[mid as usize] < target {
        return find_floor(&num_arr, target, mid +1, end, num_arr[mid as usize]);
    }else{
        return find_floor(&num_arr, target, start, mid -1, result);

    }
}

fn find_ciel(num_arr:&Vec<i32>, target:i32, start: i32, end:i32, result:i32) -> i32{
    let mid = (start + end )/ 2;
    if start > end{
        return result;
    }
    if num_arr[mid as usize] == target {
        return target;
    }
    if num_arr[mid as usize] < target {
        return find_ciel(&num_arr, target, mid +1, end, result);
    }else{
        return find_ciel(&num_arr, target, start, mid -1, num_arr[mid as usize]);

    }
}

fn sqrt(num:i32, mid:i32,result:i32)->i32{
    // let mid = (start + num) / 2;
    let sq = mid * mid;
    if sq == num{
        return mid;
    }
    if sq < num{
        return mid;
    }else{
        return sqrt(num, mid-1, result);
    }
}

fn find_frequency_of_each_element(num_arr: &Vec<i32>,start: i32,end: i32,mut freq: HashMap<i32, i32>,) -> HashMap<i32, i32> {
    if start >  end {
        return freq;
    }
    let mid = (start + end) / 2;
    if num_arr[start as usize] == num_arr[mid as usize]{
        // Check if the key exists and update it if needed
        if let Some(existing_value) = freq.get_mut(&num_arr[mid as usize]) {
            *existing_value = *existing_value + (mid-start) + 1; // Update the value
        } else {
            // Key doesn't exist, so insert a new key-value pair
            freq.insert(num_arr[mid as usize], (mid-start) + 1);
        }
        return find_frequency_of_each_element(num_arr, mid + 1, (num_arr.len() -1) as i32 , freq);
    }else{
        return find_frequency_of_each_element(num_arr, start, mid, freq)
    }
}

fn find_odd_occurance(num_arr: &Vec<i32>,start: i32,end: i32)-> i32{
    // nums[] = { 2, 2, 1, 1, 3, 3, 2, 2, 4, 4, 3, 1, 1 }
    // pos[] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 }
    let mid = (start + end) / 2;
    println!("mid {}, start {}, end {}",mid, start, end);
    if start > end {
        return -1;
    }
    if start == end {
        return num_arr[start as usize];
    }
    if mid % 2 == 0 {
        if num_arr[mid as usize] == num_arr[(mid + 1) as usize]{
            return find_odd_occurance(num_arr, mid + 2, end);
        }else{
            return find_odd_occurance(num_arr, start, mid-1);
        }
    }else{
        if num_arr[mid as usize] == num_arr[(mid - 1) as usize]{
            return find_odd_occurance(num_arr, mid + 1, end);
        }else{
            return find_odd_occurance(num_arr, start, mid - 1);
        }
    }
}


fn new_binary_search(num_arr: &Vec<i32>,target:i32,start: i32,end: i32)->i32 {
    if start > end{
        return -1;
    }
    let mid = (start+end) / 2;
    if num_arr[mid as usize] == target{
        return target;
    }
    if num_arr[mid as usize] > target{
        return  new_binary_search(&num_arr, target, start, mid -1);
    }else{
        return  new_binary_search(num_arr, target, mid +1, end);
    }

}
fn find_pair(num_arr: &Vec<i32>, sub:i32){
    for i in num_arr.iter(){
        let result = new_binary_search(num_arr, i - sub, 0, num_arr.len() as i32);
        if result != -1{
            println!("{},{}", i,result)
        }
    }
}

fn find_closest(num_arr: &Vec<i32>, k:i32, target:i32, start: i32, end: i32)->&[i32]{
    // nums = [10, 12, 15, 17, 18, 20, 25]
    // println!("start {}, end {}",start, end);
    if (end-start) < k{
        return &num_arr[(start as usize)..((end+1) as usize)];
    }

    if (num_arr[start as usize] - target).abs() > (num_arr[end as usize] - target).abs(){
        return find_closest(num_arr, k, target, start + 1,end)
    }else{
        return find_closest(num_arr, k, target, start,end -1)

    }

}