// https://leetcode.com/problems/merge-sorted-array/

export function merge_sorted_array(nums1: number[], m: number, nums2: number[], n: number) {
  let p1 = m - 1;
  let p2 = n - 1;
  let i = m + n - 1;
  while (i >= 0) {
    if (p1 < 0 || nums1[p1] <= nums2[p2]) {
      nums1[i] = nums2[p2]
      p2--
    } else if (p2 < 0 || nums1[p1] > nums2[p2]) {
      nums1[i] = nums1[p1]
      p1--
    }
    i--
  }

  return nums1
}
