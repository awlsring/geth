$version: "2.0"

namespace awlsring.geth.control

resource Group {
    identifiers: { id: GroupId },
    read: DescribeGroup,
    list: ListGroups,
    create: CreateGroup,
    delete: DeleteGroup,
}

string GroupId

structure GroupSummary {
    @required
    id: GroupId

    @required
    name: String

    platform: String

    location: String
}

list GroupSummaries {
    member: GroupSummary
}