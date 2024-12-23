part2 solved by establishing that the largest group is of size 13. Done by making a list 

cat input23.txt | awk -F\- '{print $2"-"$1}' > /tmp/l2
cat input23.txt >> /tmp/l2
sort /tmp/l2  | cut -c1-2| uniq -c| sort -n | less

in fact, all the groups are of size 13.

converting the graph list to dot format:
1) surround list with graph { ... }
2) replace - with --
3) add option to disable overlap for better rendering: graph [overlap=false];

neato -Tpdf -o visu.pdf visu.do


with the neato visualiser, there was one group that was clearly more isolated/less connected than the rest
