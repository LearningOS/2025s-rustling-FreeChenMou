/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    let mut n=array.len();
    quickSort(array,0,n-1)
}

fn quickSort<T: std::cmp::PartialOrd>(array: &mut [T],left:usize,right:usize){
    if left>=right{
        return
    }
    let  (mut l,mut r,mut temp) =(left,right,left);
    while l<r{
        while l<r && array[r]>=array[temp]{
            r-=1
        }
        
        while l<r && array[temp]>=array[l]{
            l+=1
        }
        if l!=r{
            array.swap(r,l);
        }
    }
    array.swap(l,temp);
    if l!=0{
        quickSort(array,left,l-1);
    }
    
    quickSort(array,l+1,right);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}